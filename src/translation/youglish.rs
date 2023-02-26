use crate::translation::GenericTranslator;

#[derive(Copy, Clone)]
pub struct Youglish {}

impl GenericTranslator for Youglish {
    fn name(&self) -> String {
        "youglish".into()
    }

    fn js_code(&self) -> String {
        include_str!("../js/youglish.js").into()
    }

    fn url(&self, text: String) -> String {
        format!("https://youglish.com/pronounce/{text}/english?")
    }
}
