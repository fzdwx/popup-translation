use crate::translation::{GenericTranslator, UrlTranslator};

#[derive(Copy, Clone)]
pub struct Youglish {}

impl GenericTranslator for Youglish {
    fn name(&self) -> String {
        "youglish".into()
    }

    fn translate(&self, text: String) -> String {
        format!("https://youglish.com/pronounce/{text}/english?")
    }
}

impl UrlTranslator for Youglish {
    fn js_code(&self) -> String {
        include_str!("../js/youglish.js").into()
    }
}
