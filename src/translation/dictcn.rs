use crate::translation::{GenericTranslator, UrlTranslator};

#[derive(Copy, Clone)]
pub struct Dictcn {}

impl GenericTranslator for Dictcn {
    fn name(&self) -> String {
        "dictcn".into()
    }

    fn translate(&self, text: String) -> String {
        format!("http://dict.cn/{text}")
    }
}

impl UrlTranslator for Dictcn {
    fn js_code(&self) -> String {
        include_str!("../js/dictcn.js").to_string()
    }
}
