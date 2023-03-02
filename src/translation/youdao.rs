use crate::translation::{Content, GenericTranslator};

#[derive(Copy, Clone)]
pub struct YouDao {}

impl GenericTranslator for YouDao {
    fn name(&self) -> String {
        "youdao".into()
    }

    fn size(&self) -> (u32, u32) {
        (655, 400)
    }

    fn js_code(&self) -> String {
        include_str!("../js/youdao.js").to_string()
    }

    fn content(&self, text: String) -> Content {
        Content::Url(format!("https://www.youdao.com/w/eng/{text}"))
    }
}
