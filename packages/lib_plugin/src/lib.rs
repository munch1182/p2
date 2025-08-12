use std::{
    ffi::{OsStr, OsString},
    fs::File,
    path::{Path, PathBuf},
};

pub const DEF_ICO_NAME: &'static str = "icon.png";
pub const DEF_WINDOW_NAME: &'static str = "window.html";
pub const DEF_LIB_NAME: &'static str = "lib.dll";

pub trait Plugin {
    /**
     * 插件名
     * 如果没有配置，则使用title默认生成
     */
    fn name(&self) -> String;
    /**
     * 插件标题，用以显示
     */
    fn title(&self) -> &str;
    /**
     * 插件版本，用于比较更新
     */
    fn version(&self) -> &str;
    /**
     * 插件图标
     */
    fn icon(&self) -> PathBuf;
    /**
     * 插件关键词, 用于模糊搜索
     * 会自动将title添加到关键词中
     */
    fn words(&self) -> KeyWords;
    /**
     * 前端窗口文件路径
     */
    fn window(&self) -> PathBuf;
    /**
     * 后端文件路径
     */
    fn libs(&self) -> PathBuf;
    /**
     * 配置窗口信息，否则使用默认值
     */
    fn window_info(&self) -> WindowInfo;
}

pub struct PluginLoader;

#[derive(serde::Deserialize, Clone)]
pub struct PluginStruct {
    name: Option<String>,
    title: String,
    version: String,
    icon: Option<String>,
    words: Option<KeyWords>,
    window: Option<String>,
    libs: Option<String>,
    window_info: Option<WindowInfo>,
    _json: Option<OsString>,
}

impl std::fmt::Debug for PluginStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let plugin: Box<&dyn Plugin> = Box::new(self);
        let _ = f.write_fmt(format_args!("PluginStruct {{ name: {:?}, title: {:?}, version: {:?}, icon: {:?}, words: {:?}, window: {:?}, libs: {:?}, window_info: {:?} }}", plugin.name(), plugin.title(), plugin.version(), plugin.icon().display(), plugin.words(), plugin.window().display(), plugin.libs(), plugin.window_info()));
        Ok(())
    }
}

impl PluginStruct {
    /**
     * 设置json文件夹路径
     * 如果没有在json文件中配置必要的文件路径，则使用此文件的父文件夹查找默认名称的文件
     */
    fn _set_json_dir<P: AsRef<OsStr>>(&mut self, p: P) {
        self._json = Some(p.as_ref().into());
    }

    // 如果json字符为路径则如果其为dir，则返回该dir；如果为文件路径，则返回其父文件夹，否则返回none
    fn _json_dir<P: AsRef<Path>>(&self, p: P) -> Option<PathBuf> {
        let path = p.as_ref();
        if path.is_dir() {
            Some(path.to_path_buf())
        } else if path.is_file() {
            path.parent().map(|x| x.to_path_buf())
        } else {
            None
        }
    }

    /**
     * @see Plugin::icon
     * @see Plugin::window
     * @see Plugin::libs
     */
    fn _some_or_dir(&self, value: Option<&String>, name: &str) -> PathBuf {
        match value {
            Some(value) => PathBuf::from(value),
            None => {
                if let Some(p) = &self._json {
                    let p = Path::new(&p);
                    let dir = self._json_dir(p);
                    if let Some(dir) = dir {
                        let dir = dir.join(name);
                        return dir;
                    }
                }

                PathBuf::from(name)
            }
        }
    }
}

impl Plugin for PluginStruct {
    fn name(&self) -> String {
        match &self.name {
            Some(name) => name.to_string(),
            None => format!(
                "plugin_{}",
                &self
                    .title()
                    .split(" ")
                    .collect::<Vec<_>>()
                    .join("_")
                    .to_lowercase()
            ),
        }
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn icon(&self) -> PathBuf {
        let icon = &self.icon;
        self._some_or_dir(icon.as_ref(), DEF_ICO_NAME)
    }

    fn words(&self) -> KeyWords {
        match &self.words {
            Some(word) => {
                let mut word = word.clone();
                let title = self.title().to_string();
                if !word.words.contains(&title) {
                    word.words.insert(0, title);
                }
                word
            }
            None => KeyWords {
                words: vec![self.title.clone()],
            },
        }
    }

    fn window(&self) -> PathBuf {
        let window = &self.window;
        self._some_or_dir(window.as_ref(), DEF_WINDOW_NAME)
    }

    fn libs(&self) -> PathBuf {
        let libs = &self.libs;
        self._some_or_dir(libs.as_ref(), DEF_LIB_NAME)
    }

    fn window_info(&self) -> WindowInfo {
        match self.window_info {
            Some(w) => w,
            None => WindowInfo {
                width: 800,
                height: 600,
            },
        }
    }
}

impl PluginLoader {
    /**
     * 从json文件中加载插件
     */
    pub fn load_plugin<P: AsRef<Path>>(json_path: P) -> Result<PluginStruct, std::io::Error> {
        let path = json_path.as_ref();
        let rdr = File::open(path)?;
        let mut res: PluginStruct = serde_json::from_reader(rdr)?;
        res._set_json_dir(path.as_os_str());
        Ok(res)
    }
}

#[derive(Debug, serde::Deserialize, Clone, Copy)]
pub struct WindowInfo {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct KeyWords {
    // 只能有两个关键字，可都是可选
    // 标题默认为一个关键字
    // 关键字只会取关联度更高的结果，且非标题的关键字匹配时会减分
    words: Vec<String>,
}

// 自定义json解析
impl<'de> serde::Deserialize<'de> for KeyWords {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let words: Result<Vec<String>, D::Error> = Vec::deserialize(deserializer);
        match words {
            Ok(words) => Ok(Self { words }),
            Err(_) => Ok(Self { words: vec![] }),
        }
    }
}

impl<'a> IntoIterator for &'a KeyWords {
    type Item = &'a str;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.words
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .into_iter()
    }
}

impl KeyWords {
    pub fn new_word<S: ToString>(word: S) -> Self {
        Self {
            words: vec![word.to_string()],
        }
    }

    pub fn new_words<S: ToString>(word1: S, word2: S) -> Self {
        Self {
            words: vec![word1.to_string(), word2.to_string()],
        }
    }

    pub fn words(&self) -> &Vec<String> {
        &self.words
    }
}
