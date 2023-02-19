mod bing;
mod dictcn;
mod youdao;
mod youglish;

use crate::{
    translation::bing::Bing, translation::dictcn::Dictcn, translation::youdao::YouDao,
    translation::youglish::Youglish,
};
use wry::application::dpi::LogicalSize;

pub enum Translator {
    WebView(Box<dyn WebViewTranslator>),
    Html,
}

impl Translator {
    pub fn name(&self) -> String {
        match self {
            Translator::WebView(translator) => translator.name(),
            Translator::Html => "html".into(), // todo
        }
    }

    pub fn size(&self) -> (u32, u32) {
        match self {
            Translator::WebView(translator) => translator.size(),
            Translator::Html => (600, 400), // todo
        }
    }

    pub fn inner_size(&self) -> LogicalSize<u32> {
        let (w, h) = self.size();
        LogicalSize::new(w, h)
    }
}

pub trait WebViewTranslator {
    fn name(&self) -> String;
    fn size(&self) -> (u32, u32) {
        (600, 400)
    }

    fn url(&self, text: String) -> String;
    fn js_code(&self) -> String;
}

pub fn get_translator(name: String) -> Translator {
    match name.as_str() {
        "youdao" => Translator::WebView(Box::new(YouDao {})),
        "dictcn" => Translator::WebView(Box::new(Dictcn {})),
        "youglish" => Translator::WebView(Box::new(Youglish {})),
        "bing" => Translator::WebView(Box::new(Bing {})),
        _ => Translator::WebView(Box::new(Bing {})),
    }
}
