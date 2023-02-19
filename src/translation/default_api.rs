use crate::translation::ApiTranslator;
use crate::translation::GenericTranslator;

pub struct DefaultApiTranslator {}

impl GenericTranslator for DefaultApiTranslator {
    fn name(&self) -> String {
        return "default".to_owned();
    }
}

impl ApiTranslator for DefaultApiTranslator {
    fn html(&self, text: String) -> String {
        format!(
            r#"
        <!DOCTYPE html>
        <html>
        <head>
        <meta charset="utf-8">
        <title>Default</title>
        </head>
        <body>
        <h1>Default</h1>
        <p>{}</p>
        </body>
        </html>
        "#,
            text
        )
    }
}
