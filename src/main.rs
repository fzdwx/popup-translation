mod platform;

use wry::application::dpi::{LogicalPosition, Position};
use wry::application::window::WindowAttributes;
use crate::platform::{get_translator, Translator};

fn main() -> wry::Result<()> {
    let platform = get_translator("123".into());
    let word = "翻译器".into();
    show(platform, word)
}

fn show(translator: Box<dyn Translator>, word: String) -> wry::Result<()> {
    use wry::{
        application::{
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder,
        },
        webview::WebViewBuilder,
    };

    #[cfg(target_os = "macos")]
        let user_agent_string = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.1 Safari/605.1.15";
    #[cfg(target_os = "windows")]
        let user_agent_string = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36";
    #[cfg(target_os = "linux")]
        let user_agent_string = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36";

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(translator.name())
        .with_inner_size(translator.inner_size())
        .with_resizable(false)
        .with_focused(true)
        // .with_always_on_top(true)
        .with_position(Position::Logical(LogicalPosition::new(-100.0, 100.0)))
        .build(&event_loop)?;
    let webview = WebViewBuilder::new(window)?
        .with_url(translator.url(word).as_str())?
        .with_user_agent(user_agent_string)
        .with_initialization_script(translator.js_code().as_str())
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