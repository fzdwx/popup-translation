use crate::translation::Translator;

#[derive(Copy, Clone)]
pub struct Dictcn {}

impl Translator for Dictcn {
    fn name(&self) -> String {
        "dictcn".into()
    }

    fn url(&self, word: String) -> String {
        format!("http://dict.cn/{word}")
    }

    fn js_code(&self) -> String {
        include_str!("../js/dictcn.js").to_string()
    }
}