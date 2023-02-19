use crate::translation::{GenericTranslator, UrlTranslator};

#[derive(Copy, Clone)]
pub struct Bing {}

impl GenericTranslator for Bing {
    fn name(&self) -> String {
        "bing".into()
    }

    fn size(&self) -> (u32, u32) {
        (545, 400)
    }

    fn translate(&self, text: String) -> String {
        format!("http://www.bing.com/dict/search?mkt=zh-cn&q={text}")
    }
}

impl UrlTranslator for Bing {
    fn js_code(&self) -> String {
        include_str!("../js/bing.js").to_string()
    }
}
