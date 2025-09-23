use crate::prelude::Result;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

/// 追加写入文件
pub trait WriteAppendExt {
    fn write_append(&mut self, buf: &[u8]) -> Result<()>;
}

impl<P: AsRef<Path>> WriteAppendExt for P {
    /// 将数据追加到文件中，只适合一次性写入
    fn write_append(&mut self, buf: &[u8]) -> Result<()> {
        let path = self.as_ref();
        path.create_parent()?;
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        file.write_all(buf)?;
        Ok(())
    }
}

/// 添加多个路径
pub trait PathJoinExt {
    fn join_all<P: AsRef<Path>>(self, path: &[P]) -> PathBuf;
}

impl PathJoinExt for &Path {
    ///
    /// ```rust
    /// use libcommon::ext::PathJoinExt;
    /// use std::path::Path;
    /// let path = Path::new("\\tmp\\aaa").join_all(&["b", "a"]);
    /// println!("{:?}", path);
    /// #[cfg(target_os = "windows")]
    /// assert_eq!(path, Path::new("/tmp/aaa/b/a"));
    /// ```
    ///
    fn join_all<P: AsRef<Path>>(self, path: &[P]) -> PathBuf {
        let mut buf = self.to_path_buf();
        for p in path {
            buf.push(p.as_ref());
        }
        buf
    }
}

/// 如果文件夹不存在，则创建文件夹
pub trait FileDirCreateExt
where
    Self: Sized,
{
    /// 如果父文件夹不存在，则创建父文件夹
    ///
    /// 比如创建文件时，保证父文件夹存在
    fn create_parent(self) -> Result<Self>;
    /// 如果当前文件夹不存在，则创建当前文件夹
    fn create_dir(self) -> Result<Self>;
}

impl<P: AsRef<Path>> FileDirCreateExt for P {
    fn create_parent(self) -> Result<Self> {
        let path = self.as_ref();
        if path.exists() {
            return Ok(self);
        }
        if let Some(parent) = path.parent()
            && !parent.exists()
        {
            fs::create_dir_all(parent)?;
        }
        Ok(self)
    }

    fn create_dir(self) -> Result<Self> {
        let path = self.as_ref();
        if path.exists() {
            return Ok(self);
        }
        fs::create_dir_all(path)?;
        Ok(self)
    }
}

/// 查找当前目录下满足条件的文件和文件夹
///
/// `FIND`: 判断条件
pub trait FileFinderExt<FIND: Fn(&Path) -> bool> {
    /// 递归查找当前目录下满足条件的文件
    /// 会查找所有子文件夹
    fn find_file(&self, find: FIND) -> Vec<PathBuf> {
        self.find(true, true, find)
    }
    /// 递归查找当前目录下满足条件的文件和文件夹
    ///
    /// `alldir`: 跳过文件夹检查，为[true]时，会查找所有文件夹
    /// `recursive`: 是否递归查找, 为[true]时，会递归查找所有子文件夹
    /// `find`: 判断条件
    fn find(&self, alldir: bool, recursive: bool, find: FIND) -> Vec<PathBuf>;
}

fn _find(
    path: &Path,
    alldir: bool,
    recursive: bool,
    find: &impl Fn(&Path) -> bool,
) -> Vec<PathBuf> {
    let mut results = Vec::new();

    if path.is_file() {
        if find(path) {
            results.push(path.to_path_buf());
        }
        return results;
    }

    if !path.is_dir() {
        return results;
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            // 需要对文件夹也进行判断
            if entry_path.is_file() && find(&entry_path) {
                results.push(entry_path);
            } else if entry_path.is_dir() && recursive && (alldir || find(&entry_path)) {
                results.extend(_find(&entry_path, alldir, recursive, find));
            }
        }
    }
    results
}

impl<P: AsRef<Path>, FIND: Fn(&Path) -> bool> FileFinderExt<FIND> for P {
    fn find(&self, alldir: bool, recursive: bool, find: FIND) -> Vec<PathBuf> {
        _find(self.as_ref(), alldir, recursive, &find)
    }
}

#[cfg(test)]
mod tests {
    use crate::curr_dir;

    use super::*;

    #[test]
    fn test_ext() {
        let path = Path::new("\\tmp\\aaa").join_all(&["b", "a"]);
        println!("{:?}", path);
        #[cfg(target_os = "windows")]
        assert_eq!(path, Path::new("/tmp/aaa/b/a"));
    }

    #[test]
    fn test_find() -> Result<()> {
        let parent = curr_dir!("test_a")?.create_dir()?;
        let dir = curr_dir!(&parent, "b")?.create_dir()?;
        fs::write(dir.join("a.aaa"), b"")?;
        let dir2 = dir.join("c").create_dir()?;
        fs::write(dir2.join("b.aaa"), b"")?;
        let di3 = curr_dir!("test_a", "b2")?.create_dir()?;
        fs::write(di3.join("b.aaa"), b"")?;

        let res = parent.find_file(|p| p.extension().unwrap_or_default() == "aaa");
        assert_eq!(res.len(), 3);

        let res = parent.find(false, true, |p| {
            if p.is_file() {
                p.extension().unwrap_or_default() == "aaa"
            } else if p.is_dir() {
                p.file_name()
                    .map(|s| s.to_string_lossy())
                    .unwrap_or_default()
                    .ends_with("2")
            } else {
                false
            }
        });
        assert_eq!(res.len(), 1);

        let res = parent.find(true, false, |p| p.extension().unwrap_or_default() == "aaa");
        assert_eq!(res.len(), 0);

        let _ = fs::remove_dir_all(parent);
        Ok(())
    }
}
