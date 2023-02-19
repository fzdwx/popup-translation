use crate::translation::{ApiTranslator, GenericTranslator};
use reqwest::blocking::Client;
use serde_json::json;

pub struct AiGPT {
    key: String,
}

impl AiGPT {
    pub fn new(key: Option<String>) -> Result<AiGPT, String> {
        match key {
            Some(key) => Ok(AiGPT { key }),
            None => Err("Ai: GPT requires api key".to_string()),
        }
    }
}

impl GenericTranslator for AiGPT {
    fn name(&self) -> String {
        "AiGPT".to_string()
    }
}

impl ApiTranslator for AiGPT {
    fn html(&self, text: String) -> String {
        let result = match request(text.clone(), self.key.clone()) {
            Ok(s) => s,
            Err(s) => s,
        };
        return format!(
            r#"
         <style>
         </style>
         <div>
            <div id="text">Text: {}</div>
            <div id="translation">translation: {}</div>
        </div>
        "#,
            text.clone(),
            result.clone()
        );
    }
}

fn request(text: String, api_key: String) -> Result<String, String> {
    let client = Client::new();
    let prompt = vec!["translate to auto(if zh then en, if en then zh)", &text];

    let response = client
        .post("https://api.openai.com/v1/completions")
        .json(&json!({
            "top_p": 1,
            "stop": "```",
            "temperature": 0,
            "suffix": "\n```",
            "max_tokens": 1000,
            "presence_penalty": 0,
            "frequency_penalty": 0,
            "model": "text-davinci-003",
            "prompt":prompt.join(" "),
        }))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .unwrap();

    let status_code = response.status();

    if status_code.is_success() {
        let body = response.text().unwrap();
        let body: serde_json::Value = serde_json::from_str(&body).unwrap();
        let text = body["choices"][0]["text"].as_str().unwrap().to_string();
        Ok(text)
    } else {
        Err(format!("Error: {}", status_code))
    }
}
