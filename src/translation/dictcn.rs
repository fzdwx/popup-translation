use crate::translation::{Content, GenericTranslator};

#[derive(Copy, Clone)]
pub struct Dictcn {}

impl GenericTranslator for Dictcn {
    fn name(&self) -> String {
        "dictcn".into()
    }

    fn js_code(&self) -> String {
        include_str!("../js/dictcn.js").to_string()
    }

    fn content(&self, text: String) -> Content {
        Content::Url(format!("http://dict.cn/{text}"))
    }
}
