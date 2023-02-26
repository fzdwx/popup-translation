mod ai_gpt;
mod bing;
mod dictcn;
mod google_api;
mod youdao;
mod youglish;

use crate::translation::ai_gpt::AiGPT;
use crate::translation::google_api::GoogleApi;
use crate::{
    translation::bing::Bing, translation::dictcn::Dictcn, translation::youdao::YouDao,
    translation::youglish::Youglish,
};
use wry::application::dpi::LogicalSize;
use wry::webview::{WebView, WebViewBuilder};

/// Translator
pub struct Translator {
    inner: Box<dyn GenericTranslator>,
}

impl Translator {
    pub fn name(&self) -> String {
        self.inner.name()
    }

    pub fn size(&self) -> (u32, u32) {
        self.inner.size()
    }

    pub fn icon(&self) -> &'static [u8] {
        self.inner.icon()
    }

    pub fn build_webview(&self, webview: WebViewBuilder, text: String) -> WebView {
        webview
            .with_url(self.inner.url(text).as_str())
            .unwrap()
            .with_initialization_script(self.inner.js_code().as_str())
            .build()
            .unwrap()
    }

    pub fn inner_size(&self) -> LogicalSize<u32> {
        let (w, h) = self.size();
        LogicalSize::new(w, h)
    }
}

pub trait GenericTranslator {
    fn name(&self) -> String;
    fn icon(&self) -> &'static [u8] {
        b""
    }
    fn size(&self) -> (u32, u32) {
        (600, 400)
    }

    fn js_code(&self) -> String {
        "".into()
    }

    /// format url
    fn url(&self, text: String) -> String;

    fn conetnt(&self, text: String) -> String {
        text
    }
}

pub fn get_translator(name: String, key: Option<String>) -> Result<Translator, String> {
    let t: Box<dyn GenericTranslator> = match name.as_str() {
        "youdao" => Box::new(YouDao {}),
        "dictcn" => Box::new(Dictcn {}),
        "youglish" => Box::new(Youglish {}),
        "bing" => Box::new(Bing {}),
        "ai" => AiGPT::new(key).map(|t| Box::new(t)).unwrap(),
        "google" => Box::new(GoogleApi::new()),
        _ => Box::new(Bing {}),
    };
    Ok(Translator { inner: t })
}
