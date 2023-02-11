use wry::application::dpi::PhysicalPosition;
use wry::application::error::ExternalError;
use crate::platform::{get_translator, Translator};

mod platform;

fn main() -> wry::Result<()> {
    let platform = get_translator("123".into());
    let word = "你好".into();
    show(platform, word)
}

fn show(translator: Box<dyn Translator>, word: String) -> wry::Result<()> {
    use wry::{
        application::{
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder,
            dpi::{Position},
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
        .with_position(Position::Physical(position(event_loop.cursor_position())))
        .build(&event_loop)?;

    let _webview = WebViewBuilder::new(window)?
        .with_url(translator.url(word).as_str())?
        .with_user_agent(user_agent_string)
        .with_initialization_script(translator.js_code().as_str())
        .build()?;


    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                println!("Popup translation has started!")
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

fn position(pos: Result<PhysicalPosition<f64>, ExternalError>) -> PhysicalPosition<i32> {
    match pos {
        Ok(ph) => {
            PhysicalPosition::new(ph.x as i32, ph.y as i32)
        }
        Err(_) => {
            PhysicalPosition::new(0, 0)
        }
    }
}