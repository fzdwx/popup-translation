mod youdao;
mod dictcn;
mod youglish;
mod bing;

use wry::application::dpi::LogicalSize;
use crate::platform::bing::Bing;
use crate::platform::dictcn::Dictcn;
use crate::platform::youdao::YouDao;
use crate::platform::youglish::Youglish;

pub trait Translator {
    fn name(&self) -> String;
    fn url(&self, word: String) -> String;
    fn js_code(&self) -> String;

    fn inner_size(&self) -> LogicalSize<u32> {
        let (w, h) = self.size();
        LogicalSize::new(w, h)
    }
    
    fn size(&self) -> (u32, u32) {
        (600, 400)
    }
}

pub fn get_translator(name: String) -> Box<dyn Translator> {
    match name.as_str() {
        "youdao" => Box::new(YouDao {}),
        "dictcn" => Box::new(Dictcn {}),
        "youglish" => Box::new(Youglish {}),
        "bing" => Box::new(Bing {}),
        _ => Box::new(Bing {}),
    }
}