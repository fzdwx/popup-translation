mod bing;
mod default_api;
mod dictcn;
mod youdao;
mod youglish;

use crate::{
    translation::bing::Bing, translation::default_api::DefaultApiTranslator,
    translation::dictcn::Dictcn, translation::youdao::YouDao, translation::youglish::Youglish,
};
use wry::application::dpi::LogicalSize;
use wry::webview::{WebView, WebViewBuilder};

/// Translator
pub enum Translator {
    /// load specific url and run js code clean view with translate
    Url(Box<dyn UrlTranslator>),
    /// request api to translate
    Api(Box<dyn ApiTranslator>),
}

impl Translator {
    pub fn name(&self) -> String {
        match self {
            Translator::Url(translator) => translator.name(),
            Translator::Api(translator) => translator.name(),
        }
    }

    pub fn size(&self) -> (u32, u32) {
        match self {
            Translator::Url(translator) => translator.size(),
            Translator::Api(translator) => translator.size(),
        }
    }

    pub fn build_webview(&self, webview: WebViewBuilder, text: String) -> WebView {
        return match self {
            Translator::Url(translator) => webview
                .with_url(translator.url(text).as_str())
                .unwrap()
                .with_initialization_script(translator.js_code().as_str())
                .build(),
            Translator::Api(translator) => {
                webview.with_html(translator.html(text)).unwrap().build()
            }
        }
        .unwrap();
    }

    pub fn inner_size(&self) -> LogicalSize<u32> {
        let (w, h) = self.size();
        LogicalSize::new(w, h)
    }
}

pub trait GenericTranslator {
    fn name(&self) -> String;
    fn size(&self) -> (u32, u32) {
        (600, 400)
    }
}

pub trait UrlTranslator: GenericTranslator {
    fn js_code(&self) -> String;
    /// format url
    fn url(&self, text: String) -> String;
}

pub trait ApiTranslator: GenericTranslator {
    fn html(&self, text: String) -> String;
}

pub fn get_translator(name: String) -> Translator {
    match name.as_str() {
        "youdao" => Translator::Url(Box::new(YouDao {})),
        "dictcn" => Translator::Url(Box::new(Dictcn {})),
        "youglish" => Translator::Url(Box::new(Youglish {})),
        "bing" => Translator::Url(Box::new(Bing {})),
        "api" => Translator::Api(Box::new(DefaultApiTranslator {})),
        _ => Translator::Url(Box::new(Bing {})),
    }
}
