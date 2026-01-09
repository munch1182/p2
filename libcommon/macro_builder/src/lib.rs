//!
//! 生成与字段相关的宏
//! 
//! `Default_with`, `Builder`用于创建；
//! `With`用于修改；
//! `Getter`用于获取;
//! 

use proc_macro::TokenStream;
mod builder;
mod default_with;
mod getter;
mod with;

/// 生成的`Builder`结构和其对应的`new`方法、`build`方法以及[Option]字段的`setter`方法
///
///
/// 如果该字段不是[Option]类型，则认为是必选字段，需要在`new`方法中传入；
/// 否则为其生成同名设置方法
///
/// # example
///```ignore
/// #[derive(Builder)]
/// struct User {
///    name: String,
///    age: u16,
///    agent: Option<bool>,
///    address: Option<String>,
/// }
///
/// let user = UserBuilder::new("Jack".to_string(), 22)
///    .address("Beijing".to_string())
///    .build();
/// assert!(user.address.is_some());
/// ```
#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    builder::derive_builder_impl(input)
}

/// 为属性生成`get`方法用以获取
/// 
/// # example
/// ```ignore
/// #[derive(Getter)]
/// struct User {
///    name: String,
///    age: u16,
///    #[getter(skip)]
///    agent: Option<bool>,
/// }
///
/// let user = User {
///    name: "Jack".to_string(),
///    age: 22,
///    agent: Some(true),
/// }
/// 
/// user.get_name();
/// user.get_age();
/// 
/// 
#[proc_macro_derive(Getter, attributes(getter))]
pub fn derive_getter(input: TokenStream) -> TokenStream {
    getter::derive_getter_impl(input)
}

/// 为属性生成`with`方法用以替换
///
/// # usage
/// 当字段类型为[Option]时，默认生成的`with`的传入参数为其泛型
/// 如果字段被标记为`#[with(keep)]`，则生成的with方法保留为[Option]类型
/// 如果字段被标记为`#[with(skip)]`，则不生成该字段的`with`方法
///
/// # example
///```ignore
/// #[derive(With, Default)]
/// struct User {
///    name: String,
///    #[with(skip)]
///    age: u16,
///    #[with(keep)]
///    agent: Option<bool>,
///    address: Option<String>,
/// }
///
/// let user = User::default().with_agent(None).with_address("Beijing".to_string());
/// assert!(user.agent.is_none());
/// ```
#[proc_macro_derive(With, attributes(with))]
pub fn with_builder(input: TokenStream) -> TokenStream {
    with::with_builder_impl(input)
}

/// 为属性生成`default_with`方法用生成对象
///
///
/// 如果该属性没有实现[Default]，需要手动标记`#[default_with(no_default)]`并在生成的`default_with`方法中手动传入
/// 如果该属性有无参的生成方法，可以手动标记该方法`#[default_with("Struct::fn")]`生成默认值
///
/// # usage
///```ignore
/// #[derive(Default_With)]
/// struct Data {
///     #[default_with(no_default)]
///     no_default: NoDefaultStruct,
///     #[default_with("DefaultStruct::new")]
///     default: DefaultStruct,
/// }
///
/// struct NoDefaultStruct(u8);
/// struct DefaultStruct(u8);
///
/// impl DefaultStruct {
///     fn new() -> Self {
///         Self(1)
///     }
/// }
///
/// let user = Data::default_with(NoDefaultStruct(1));
/// assert!(user.no_default.0 == 1);
/// assert!(user.default.0 == 1);
/// ```
#[proc_macro_derive(Default_With, attributes(default_with))]
pub fn derive_default_with(input: TokenStream) -> TokenStream {
    default_with::derive_default_with_impl(input)
}
