use crate::preluad::*;
pub struct Search;

impl Search {
    pub fn search(input: String) -> Result<Vec<SearchedResult>> {
        Ok(vec![])
    }
}

/**
 * 搜索结果内容
 */
#[derive(Debug)]
pub struct SearchedResult {
    title: String,           // 显示的结果
    content: Option<String>, // 描述
    item_type: ItemType,     // 类型
    icon: Link,              // 图标
}

impl SearchedResult {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> Option<&str> {
        self.content.as_deref()
    }

    pub fn item_type(&self) -> ItemType {
        self.item_type
    }

    pub fn icon(&self) -> &Link {
        &self.icon
    }
}

/**
 * 搜索结果类型
 */
#[derive(Debug, Clone, Copy)]
pub enum ItemType {
    Alias,   // 别名
    Commnad, // 命令
    File,    // 文件
    App,     // 应用
}

/**
 * 资源链接
 */
#[derive(Debug)]
pub struct Link {
    url: Option<String>,
    file: Option<String>,
}

impl Link {
    pub fn new_url<S: AsRef<String>>(url: S) -> Self {
        Self {
            url: Some(url.as_ref().to_string()),
            file: None,
        }
    }

    pub fn new_file<S: AsRef<String>>(file: S) -> Self {
        Link {
            url: None,
            file: Some(file.as_ref().to_string()),
        }
    }
}
