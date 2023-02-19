use crate::translation::GenericTranslator;

pub struct DefaultApiTranslator {}

impl GenericTranslator for DefaultApiTranslator {
    fn name(&self) -> String {
        return "default".to_owned();
    }

    fn translate(&self, text: String) -> String {
        return r#"
            <!doctype html>
            <html>
              <body style="background-color:rgba(87,87,87,0.5);">hello</body>
              <script>
                window.onload = function() {
                  document.body.innerText = `hello, ${navigator.userAgent}`;
                };
              </script>
            </html>"#.to_string();
    }
}
