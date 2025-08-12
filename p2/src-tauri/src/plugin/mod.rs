use crate::{err, preluad::*, utils::OptOrExt};
use std::{
    collections::HashMap,
    ffi::OsStr,
    hash::{DefaultHasher, Hash, Hasher},
    path::{Path, PathBuf},
};

use lib_plugin::*;

#[derive(Debug)]
pub struct PluginManager<'a> {
    // 插件要保存的路径，加载的插件会复制到该路径下
    plugin_dir: &'a Path,
    plugins: HashMap<String, PluginWithId>,
}

impl IntoIterator for PluginManager<'_> {
    type Item = PluginWithId;

    type IntoIter = std::collections::hash_map::IntoValues<String, PluginWithId>;

    fn into_iter(self) -> Self::IntoIter {
        self.plugins.into_values()
    }
}

/**
 * 包裹PluginStruct，并添加id
 */
#[derive(Debug, Clone)]
pub struct PluginWithId {
    pub id: u64,
    pub plugin: PluginStruct,
}

impl<'a> PluginManager<'a> {
    /**
     * 传入插件目录，创建一个插件管理器
     * 其它位置的插件会被复制到该路径以保存，并以其它形式使用
     */
    fn new<S: AsRef<Path>>(plugin_dir: &'a S) -> Self {
        Self {
            plugin_dir: plugin_dir.as_ref(),
            plugins: HashMap::new(),
        }
    }

    /**
     * 扫描dir文件夹，加载其中的有效插件
     * 有效插件：有任意名称的json配置文件，且其中至少有关键配置
     */
    pub fn sacn_plugin<P: AsRef<Path>>(&mut self, dir: P) -> Result<()> {
        let path = dir.as_ref();
        if !path.exists() {
            return Err(err!("{:?}不存在", path));
        }
        if !path.is_dir() {
            return Err(err!("{:?}不是一个目录", path));
        }
        for entry in path.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if let Some(p) = path.to_str() {
                if path.is_file() {
                    let res = self.load_plugin_by_json(p);
                } else if path.is_dir() {
                    let _ = self.sacn_plugin(p);
                }
            }
        }
        Ok(())
    }

    /**
     * 加载该插件
     */
    fn handle_plugin(&mut self, plugin: PluginStruct) -> Result<()> {
        let withid = PluginWithId::new(plugin);
        let name = withid.plugin.name();
        log!("加载插件：{}({:?})", &name, withid.id);
        self.plugins.insert(name.to_string(), withid);
        Ok(())
    }

    /**
     * 加载一个插件，传入配置文件
     * 如果有效，则添加或者更新插件
     * 否则，则忽略
     */
    pub fn load_plugin_by_json<P: AsRef<Path>>(&mut self, plugin: P) -> Result<()> {
        let plugin = PluginLoader::load_plugin(plugin)?;
        self.handle_plugin(plugin);
        Ok(())
    }
}

trait PluginIDExt {
    /**
     * 通过name、words计算插件唯一id
     */
    fn calculate_id(&self) -> u64;
}

impl PluginIDExt for &dyn Plugin {
    fn calculate_id(&self) -> u64 {
        let mut hash = DefaultHasher::new();
        self.name().hash(&mut hash);
        self.words().into_iter().for_each(|x| x.hash(&mut hash));
        hash.finish()
    }
}
impl PluginIDExt for &PluginStruct {
    fn calculate_id(&self) -> u64 {
        let p: &dyn Plugin = *self;
        p.calculate_id()
    }
}

trait PluginVerify {
    /**
     * 验证插件文件位置是否有效
     */
    fn is_valid(&self) -> bool;
}

impl PluginVerify for &dyn Plugin {
    fn is_valid(&self) -> bool {
        return is_file_valid(self.icon())
            && is_file_valid(self.window())
            && is_file_valid(self.libs());
    }
}

impl PluginVerify for PluginStruct {
    fn is_valid(&self) -> bool {
        let p: &dyn Plugin = self;
        p.is_valid()
    }
}

fn is_file_valid(path: PathBuf) -> bool {
    return path.exists() && path.is_file();
}

impl PluginWithId {
    fn new(plugin: PluginStruct) -> Self {
        let id = (&plugin).calculate_id();
        Self { id, plugin }
    }
}

impl Into<PluginStruct> for &PluginWithId {
    fn into(self) -> PluginStruct {
        self.plugin.clone()
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_plugin() {
        println!("test plugin");
        let curr = std::env::current_dir().unwrap();
        let parent = curr.parent().unwrap();
        let dir = parent.join("test_plugin_dir");
        let workdir = dir.join("workdir");
        println!("{:?}, {:?}", dir, workdir);

        let mut manager = PluginManager::new(&workdir);
        let res = &manager.sacn_plugin(&dir);
        for (name, withid) in &manager.plugins {
            let plugin: PluginStruct = withid.into();
            let plugin = Box::new(plugin);
            println!("{:?}: {:?}", name, plugin);
        }
        assert!(res.is_ok());
        let a = &manager
            .into_iter()
            .filter(|x| {
                let stuct: PluginStruct = x.into();
                stuct.is_valid()
            })
            .collect::<Vec<PluginWithId>>();
        println!("有效：{:?}", a);
    }
}
