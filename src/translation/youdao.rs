use crate::translation::WebViewTranslator;

#[derive(Copy, Clone)]
pub struct YouDao {}

impl WebViewTranslator for YouDao {
    fn name(&self) -> String {
        "youdao".into()
    }

    fn url(&self, word: String) -> String {
        format!("https://www.youdao.com/w/eng/{word}")
    }

    fn js_code(&self) -> String {
        include_str!("../js/youdao.js").to_string()
    }
    fn size(&self) -> (u32, u32) {
        (655, 400)
    }
}
