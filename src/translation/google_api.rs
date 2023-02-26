///
/// curl 'https://translate.googleapis.com/translate_a/single?client=gtx&sl=auto&tl=zh-CN&dj=1&dt=t&dt=bd&dt=qc&dt=rm&dt=ex&dt=at&dt=ss&dt=rw&dt=ld&q=Look%20up%20our%20workspace.&tk=855086.855086' \
/// -H 'authority: translate.googleapis.com' \
/// -H 'accept: */*' \
/// -H 'accept-language: zh,en;q=0.9' \
/// -H 'sec-ch-ua: "Chromium";v="110", "Not A(Brand";v="24", "Google Chrome";v="110"' \
/// -H 'sec-ch-ua-mobile: ?0' \
/// -H 'sec-ch-ua-platform: "Linux"' \
/// -H 'sec-fetch-dest: empty' \
/// -H 'sec-fetch-mode: cors' \
/// -H 'sec-fetch-site: none' \
/// -H 'user-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36' \
/// -H 'x-client-data: CIW2yQEIo7bJAQipncoBCP3zygEIkqHLAQj8qswBCJr+zAEIwIbNAQ==' \
/// --compressed
///
use crate::translation::GenericTranslator;
use reqwest::{blocking::Client, Error};
use serde::{Deserialize, Serialize};

const ICON: &[u8] = include_bytes!("../asset/google.png");

pub struct GoogleApi {
    html: String,
}

impl GoogleApi {
    pub fn new() -> Self {
        Self {
            html: include_str!("../html/google.html").to_string(),
        }
    }

    fn html(&self, text: String) -> String {
        let html = format!("{}", self.html);
        match request(text) {
            Ok(resp) => {
                let (orig, trans) = resp.get_info();
                html.replace("$orig", orig.as_str())
                    .replace("$trans", trans.as_str())
                    .replace("$class", "ok")
            }
            Err(err) => html
                .replace("$err", err.to_string().as_str())
                .replace("$class", "err"),
        }
    }
}

impl GenericTranslator for GoogleApi {
    fn name(&self) -> String {
        "google".into()
    }

    fn icon(&self) -> &'static [u8] {
        ICON
    }

    fn js_code(&self) -> String {
        "".to_string()
    }

    fn url(&self, text: String) -> String {
        format!("wry://dev/google?q={text}")
    }
}

/// 判断一句话是中文多还是英文多
fn is_chinese(text: &str) -> bool {
    let mut count = 0;
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            count += 1;
        }
    }
    count < text.len() / 2
}

/// 根据语言判断翻译的语言
fn get_tl(text: &str) -> String {
    if is_chinese(text) {
        "en".to_string()
    } else {
        "zh-CN".to_string()
    }
}

/// request google translate api
fn request(text: String) -> Result<TranslationResponse, Error> {
    let tl = get_tl(&text);
    let query = format!("?client=gtx&sl=auto&tl={tl}&dj=1&dt=t&dt=bd&dt=qc&dt=rm&dt=ex&dt=at&dt=ss&dt=rw&dt=ld&q={text}");
    let url = format!("https://translate.googleapis.com/translate_a/single{query}");

    Client::new().get(url).send()?.json()
}

#[derive(Debug, Deserialize, Serialize)]
struct TranslationResponse {
    sentences: Vec<Sentence>,
    src: String,
    alternative_translations: Option<Vec<AlternativeTranslation>>,
    confidence: f32,
    ld_result: LDResult,
    examples: Option<Examples>,
}

impl TranslationResponse {
    fn get_info(&self) -> (String, String) {
        if let Some(sentence) = self.sentences.first() {
            let orig = if let Some(orig) = &sentence.orig {
                orig.to_string()
            } else {
                "".to_string()
            };
            let trans = if let Some(trans) = &sentence.trans {
                trans.to_string()
            } else {
                "".to_string()
            };
            (orig, trans)
        } else {
            ("".to_string(), "".to_string())
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Sentence {
    trans: Option<String>,
    orig: Option<String>,
    backend: Option<i32>,
    translit: Option<String>,
    src_translit: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AlternativeTranslation {
    src_phrase: String,
    alternative: Vec<Alternative>,
    srcunicodeoffsets: Vec<UnicodeOffset>,
    raw_src_segment: String,
    start_pos: i32,
    end_pos: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Alternative {
    word_postproc: String,
    score: i32,
    has_preceding_space: bool,
    attach_to_next_token: bool,
    backends: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
struct UnicodeOffset {
    begin: i32,
    end: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct LDResult {
    srclangs: Vec<String>,
    srclangs_confidences: Vec<f32>,
    extended_srclangs: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Examples {
    example: Vec<Example>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Example {
    text: String,
    source_type: i32,
    definition_id: String,
}

#[cfg(test)]
mod tests {
    use crate::translation::google_api::request;

    #[test]
    fn test_request_api() {
        let res = request("hello".into());
        println!("{:#?}", res);
        // assert!(res.is_ok());
    }

    #[test]
    fn test_ico() {
        // load ./assets/google.ico
        // to base64
        // let ico = include_str!("../asset/google.txt");
        // println!("{}", ico);
    }
}
