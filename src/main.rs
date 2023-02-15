use crate::translation::{ get_translator, Translator };
use clap::{ arg, Parser };
use std::num::ParseIntError;
use std::{ collections::HashMap, str::FromStr };
use wry::{
    application::window::WindowId,
    application::{
        accelerator::Accelerator,
        dpi::PhysicalPosition,
        error::ExternalError,
        event::{ Event, StartCause, WindowEvent },
        event_loop::{ ControlFlow, EventLoop, EventLoopWindowTarget },
        global_shortcut::ShortcutManager,
        window::WindowBuilder,
    },
    webview::WebView,
    webview::WebViewBuilder,
};

mod clipboard;
mod translation;

/// Popup translation
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Text to be translated, if is None, then is daemon mode.
    text: Option<String>,

    /// Platform to be used, available platforms are: bing, dictcn, youdao, youglish
    ///
    /// --platform=bing
    #[arg(short, long, default_value = "bing")]
    platform: String,

    /// Brings up the shortcut key for the translation window
    ///
    /// --show=alt+s
    #[arg(long, default_value = "Ctrl+Alt+c")]
    show: String,

    /// Override the position for the initial window launched by this process.
    ///
    /// --position=10,10   => x=10, y=10
    /// --position=123,asd => cursor position
    #[arg(long, verbatim_doc_comment, value_parser = PositionArg::parse, default_value = "cursor")]
    position: PositionArg,
}

fn main() -> wry::Result<()> {
    let args: Args = Args::parse();
    let mut prev_id = None;
    let mut webviews = HashMap::new();

    let event_loop = EventLoop::new();

    let mut hotkey_manager = ShortcutManager::new(&event_loop);
    let shortcut_show = Accelerator::from_str(args.show()).unwrap();

    if args.run_once() {
        let (id, webview) = show(
            &event_loop,
            get_translator(args.platform()),
            args.text(),
            args.position()
        );
        webviews.insert(id, webview);
    } else {
        hotkey_manager.register(shortcut_show.clone()).unwrap();
    }
    event_loop.run(move |event, event_loop, control_flow| {
        let args = args.clone();

        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => { println!("Popup translation has started!") }
            Event::GlobalShortcutEvent(hotkey_id) => {
                // remove previous window
                if let Some(id) = prev_id {
                    webviews.remove(&id);
                }

                if hotkey_id == shortcut_show.clone().id() {
                    let (id, webview) = show(
                        event_loop,
                        get_translator(args.platform()),
                        args.text(),
                        args.position()
                    );
                    prev_id = Some(id);
                    webviews.insert(id, webview);
                }
            }

            Event::WindowEvent { event, window_id, .. } =>
                match event {
                    WindowEvent::CloseRequested => {
                        webviews.remove(&window_id);

                        if args.run_once() {
                            *control_flow = ControlFlow::Exit;
                        }

                        // if webviews.is_empty() {
                        //     *control_flow = ControlFlow::Exit
                        // }
                    }
                    _ => (),
                }
            _ => (),
        }
    });
}

fn show<T: 'static>(
    event_loop: &EventLoopWindowTarget<T>,
    translator: Box<dyn Translator>,
    text: String,
    position: PositionArg
) -> (WindowId, WebView) {
    #[cfg(target_os = "macos")]
    let user_agent_string =
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.1 Safari/605.1.15";
    #[cfg(target_os = "windows")]
    let user_agent_string =
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36";
    #[cfg(target_os = "linux")]
    let user_agent_string =
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36";

    let window = WindowBuilder::new()
        .with_title(translator.name())
        .with_inner_size(translator.inner_size())
        .with_resizable(false)
        .with_focused(true)
        .with_position(position.to_wry_position(|| event_loop.cursor_position()))
        .build(event_loop)
        .unwrap();

    let window_id = window.id();
    let webview = WebViewBuilder::new(window)
        .unwrap()
        .with_url(translator.url(text).as_str())
        .unwrap()
        .with_user_agent(user_agent_string)
        .with_initialization_script(translator.js_code().as_str())
        .build()
        .unwrap();

    (window_id, webview)
}

#[derive(Debug, Clone, Default)]
pub enum PositionArg {
    Coordinate(i32, i32),

    #[default]
    Cursor,
}

impl PositionArg {
    pub fn parse(str: &str) -> Result<Self, ParseIntError> {
        let mut pairs = str.split(',');

        let x = pairs.next().unwrap_or_default();
        let y = pairs.next().unwrap_or_default();

        if x.is_empty() || y.is_empty() {
            return Ok(Self::Cursor);
        }

        let x = match x.parse::<i32>() {
            Ok(x) => x,
            Err(_) => {
                return Ok(Self::Cursor);
            }
        };

        let y = match y.parse::<i32>() {
            Ok(y) => y,
            Err(_) => {
                return Ok(Self::Cursor);
            }
        };

        Ok(Self::Coordinate(x, y))
    }

    /// convert to wry position
    pub fn to_wry_position<T>(&self, x: T) -> PhysicalPosition<i32>
        where T: FnOnce() -> Result<PhysicalPosition<f64>, ExternalError>
    {
        match self {
            Self::Coordinate(x, y) => PhysicalPosition::new(*x, *y),
            Self::Cursor => Self::position_map(x()),
        }
    }

    fn position_map(pos: Result<PhysicalPosition<f64>, ExternalError>) -> PhysicalPosition<i32> {
        match pos {
            Ok(ph) => PhysicalPosition::new(ph.x as i32, ph.y as i32),
            Err(_) => PhysicalPosition::new(0, 0),
        }
    }
}

impl Args {
    /// get text. If text is not set, read from clipboard
    pub fn text(&self) -> String {
        self.text.clone().unwrap_or_else(|| clipboard::read_text().unwrap())
    }

    /// get platform
    pub fn platform(&self) -> String {
        self.platform.clone()
    }

    /// If set, the translation will be shown once and the program will exit
    pub fn run_once(&self) -> bool {
        self.text.is_some()
    }

    /// get shortcut to show translation
    pub fn show(&self) -> &str {
        self.show.as_str()
    }

    /// get position
    pub fn position(&self) -> PositionArg {
        self.position.clone()
    }
}