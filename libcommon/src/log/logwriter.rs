use crate::if_feature;

/**
 * 当写入所有日志后，关闭日志
 */
#[allow(unused)]
pub fn log_flush() {
    flush();
}

if_feature!(not("logfile") =>
    use crate::prelude::Result;
    /**
     * 日志数据传递
     */
    pub(crate) fn write<S: Into<String>>(_: S) -> Result<()> {
        Ok(())
    }

    pub(crate) fn flush() {
    }

);

if_feature!("logfile" =>
    use std::{
        path::{Path, PathBuf},
        sync::{
            OnceLock, RwLock,
            atomic::{AtomicUsize, Ordering},
        },
    };

    use crate::{ext::WriteAppendExt, log::log_setup_result, newerr, prelude::*};
    use crossbeam_channel::{Receiver, Sender, bounded};

    /**
     * 日志数据传递
     */
    pub(crate) fn write<S: Into<String>>(s: S) -> Result<()> {
        match LOG_SENDER.get() {
            Some(tx) => {
                tx.send(s.into())?;
            }
            None => {},
        }
        Ok(())
    }

    /**
     * 当写入所有日志后，关闭日志
     */
    pub(crate) fn flush() {
        let _ = write("");
    }

    /**
     * 用于收集日志写入文件的线程
     * 会占用该线程
     */
    pub trait LogWriterTask {
        /**
         * 执行日志写入任务
         */
        fn spawn<F>(&self, log: F)
        where
            F: Future + Send + 'static,
            F::Output: Send + 'static;
    }

    ///
    ///
    /// 初始化日志显示
    /// 并将日志写入文件
    /// dir: 日志文件目录，日志文件会自动分文件存储
    ///     每次打开会创建新文件；每一份文件超过最大值会自动分文件存储
    ///
    pub fn log_setup_with_writer<P: AsRef<Path>>(executor: &impl LogWriterTask, dir: P) {
        let result = log_setup_result();
        // 已经设置过
        if result.is_err() {
            return;
        }

        let (tx, rx) = bounded::<String>(5);

        // 失败说明已经设置了
        let _ = LOG_SENDER.set(tx).map_err(|e| newerr!("set err {:?}", e));

        let runner = LogRunner::new(dir.as_ref().to_path_buf(), rx);
        executor.spawn(async move { runner.run() });
    }

    /**
     * OnceLock允许安全的一次性初始化
     * 优点是之后可以进行无锁读取
     * 缺点是初始化后无法对值进行修改
     */
    static LOG_SENDER: OnceLock<Sender<String>> = OnceLock::new();

    struct LogRunner {
        /**
         * 日志文件目录
         */
        dir: RwLock<PathBuf>,
        /**
         * 日志数据接收
         */
        rx: RwLock<Receiver<String>>,
        /**
         * 日志文件最大大小
         */
        max_size: usize,
        /**
         * 当前文件
         */
        curr_file: RwLock<Option<PathBuf>>,
        /**
         * 当前文件大小
         */
        curr_size: AtomicUsize,
    }

    impl LogRunner {
        pub(crate) fn new(p: PathBuf, rx: Receiver<String>) -> Self {
            Self {
                dir: RwLock::new(p),
                rx: RwLock::new(rx),
                max_size: 1024 * 1024 * 5,
                curr_file: RwLock::new(None),
                curr_size: AtomicUsize::new(0),
            }
        }

        pub(crate) fn run(&self) {
            if let Ok(rx) = self.rx.read() {
                loop {
                    match rx.recv() {
                        Ok(s) => {
                            // 空白字符作为退出机制 // 日志因为附加信息的存在，不可能是空白字符
                            if s.is_empty() {
                                break;
                            }
                            let res = self.write(s);
                            if let Err(e) = res {
                                eprintln!("write log error: {e}");
                            }
                        }
                        std::result::Result::Err(e) => {
                            println!("log runner exit: {e}");
                            break;
                        }
                    }
                }
            }
        }

        fn write(&self, s: String) -> Result<()> {
            let mut need_new_file = false;
            let mut curr_size = self.curr_size.load(Ordering::Relaxed);
            let mut file_path: Option<PathBuf> = None;
            // 代码分段减少锁持有时间
            {
                let curr_file = self.curr_file.read().map_err_ext()?;
                let curr_size = self.curr_size.load(Ordering::Relaxed);
                match curr_file.as_ref() {
                    Some(curr) => {
                        //大致计数，并不强制对齐大小
                        let newlen = curr_size + s.len();
                        file_path = Some(curr.to_path_buf());
                        if newlen > self.max_size {
                            need_new_file = true;
                        }
                    }
                    None => need_new_file = true,
                }
            }

            if need_new_file {
                let new_file = self.new_file()?;
                let mut currlock = self.curr_file.write().map_err_ext()?;
                *currlock = Some(new_file.clone());
                file_path = Some(new_file.clone());
                curr_size = 0;
            }

            if let Some(mut path) = file_path {
                let time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                let new_s = format!("{time}  {s}\n");
                // IO操作时不持有锁
                path.write_append(new_s.as_bytes())?;

                self.curr_size.store(curr_size + s.len(), Ordering::Relaxed);
            }
            Ok(())
        }

        fn new_file(&self) -> Result<PathBuf> {
            let path = self.dir.read().map_err_ext()?.clone();
            let name = chrono::Local::now()
                .format("log_%Y%m%d%H%M.txt")
                .to_string();
            Ok(path.join(name))
        }
    }

);
