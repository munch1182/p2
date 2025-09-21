use macro_builder::{Builder, Default_With, With};

pub fn main() {
    let resp: Resp<'_, u8> = RespBuilder::new(0).data(1).build();
    assert!(resp.data == Some(1));
    let resp = resp.with_code(1).with_data(None);
    assert!(resp.data.is_none());
    assert!(resp.msg.is_none());

    let user = Data::default_with(NoDefaultStruct(1));
    assert!(user.no_default.0 == 1);
    assert!(user.default.0 == 1);
}

#[derive(Builder, With)]
struct Resp<'a, T> {
    code: u8,
    msg: Option<&'a str>,
    #[with(keep)]
    data: Option<T>,
}

#[derive(Default_With)]
struct Data {
    #[default_with(no_default)]
    no_default: NoDefaultStruct,
    #[default_with("DefaultStruct::new")]
    default: DefaultStruct,
}

struct NoDefaultStruct(u8);
struct DefaultStruct(u8);

impl DefaultStruct {
    pub fn new() -> Self {
        DefaultStruct(1)
    }
}
