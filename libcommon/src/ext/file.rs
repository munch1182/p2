use crate::prelude::Result;
use std::{fs, io::Write, path::Path};

/**
 * 追加写入文件
 */
pub trait WriteAppendExt {
    fn write_append(&mut self, buf: &[u8]) -> Result<()>;
}

impl<P: AsRef<Path>> WriteAppendExt for P {
    /**
     * 将数据追加到文件中
     */
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

/**
 * 如果文件夹不存在，则创建文件夹
 */
pub trait FileDirCreateExt
where
    Self: Sized,
{
    fn create_parent(self) -> Result<Self>;
    fn create_dir(self) -> Result<Self>;
}

impl<P: AsRef<Path>> FileDirCreateExt for P {
    /**
     * 如果父文件夹不存在，则创建父文件夹
     * 比如创建文件时，保证父文件夹存在
     */
    fn create_parent(self) -> Result<Self> {
        let path = self.as_ref();
        if path.exists() {
            return Ok(self);
        }
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        Ok(self)
    }

    /**
     * 如果当前文件夹不存在，则创建当前文件夹
     */
    fn create_dir(self) -> Result<Self> {
        let path = self.as_ref();
        if path.exists() {
            return Ok(self);
        }
        fs::create_dir_all(path)?;
        Ok(self)
    }
}
