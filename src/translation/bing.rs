use crate::translation::Translator;

#[derive(Copy, Clone)]
pub struct Bing {}

impl Translator for Bing {
    fn name(&self) -> String {
        "bing".into()
    }

    fn url(&self, word: String) -> String {
        format!("http://www.bing.com/dict/search?mkt=zh-cn&q={word}")
    }

    fn js_code(&self) -> String {
        include_str!("../js/bing.js").to_string()
    }

    fn size(&self) -> (u32, u32) {
        (545, 400)
    }
}