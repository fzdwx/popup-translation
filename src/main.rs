mod platform;

use crate::platform::{get_translator, Translator};

fn main() -> wry::Result<()> {
    let platform = get_translator("123".into());
    let word = "翻译器".into();
    show(platform, word)
}

fn show(p: Box<dyn Translator>, word: String) -> wry::Result<()> {
    use wry::{
        application::{
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder,
        },
        webview::WebViewBuilder,
    };

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(p.name())
        .build(&event_loop)?;
    let webview = WebViewBuilder::new(window)?
        .with_url(p.url(word).as_str())?
        .with_initialization_script(p.js_code().as_str())
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                println!("Wry has started!")
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}