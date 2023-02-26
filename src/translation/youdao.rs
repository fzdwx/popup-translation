use crate::translation::GenericTranslator;

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

    fn url(&self, text: String) -> String {
        format!("https://www.youdao.com/w/eng/{text}")
    }
}
