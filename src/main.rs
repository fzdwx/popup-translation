use crate::translation::{get_translator, Translator};
use clap::{arg, Parser};
use std::num::ParseIntError;
use std::{collections::HashMap, str::FromStr};
use wry::application::platform::unix::EventLoopWindowTargetExtUnix;
use wry::{
    application::window::WindowId,
    application::{
        accelerator::Accelerator,
        dpi::PhysicalPosition,
        error::ExternalError,
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
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
    /// --position=cursor  => cursor position
    /// --position=tr      => top right
    /// --position=tl      => top left
    /// --position=tc      => top center
    /// --position=c       => center
    /// --position=br      => bottom right
    /// --position=bc      => bottom center
    /// --position=bl      => bottom left
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
            args.position(),
        );
        webviews.insert(id, webview);
    } else {
        hotkey_manager.register(shortcut_show.clone()).unwrap();
    }

    event_loop.run(move |event, event_loop, control_flow| {
        let args = args.clone();

        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                println!("Popup translation has started!")
            }
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
                        args.position(),
                    );
                    prev_id = Some(id);
                    webviews.insert(id, webview);
                }
            }

            Event::WindowEvent {
                event, window_id, ..
            } => match event {
                WindowEvent::CloseRequested => {
                    webviews.remove(&window_id);

                    if args.run_once() {
                        *control_flow = ControlFlow::Exit
                    }

                    // if webviews.is_empty() {
                    //     *control_flow = ControlFlow::Exit
                    // }
                }
                _ => (),
            },
            _ => (),
        }
    });
}

fn show<T: 'static>(
    event_loop: &EventLoopWindowTarget<T>,
    translator: Box<dyn Translator>,
    text: String,
    position: PositionArg,
) -> (WindowId, WebView) {
    #[cfg(target_os = "macos")]
        let user_agent_string = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.1 Safari/605.1.15";
    #[cfg(target_os = "windows")]
        let user_agent_string = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36";
    #[cfg(target_os = "linux")]
        let user_agent_string = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36";

    let window = WindowBuilder::new()
        .with_title(translator.name())
        .with_inner_size(translator.inner_size())
        .with_resizable(false)
        .with_focused(true)
        // .with_position(position.to_wry_position(|| event_loop.cursor_position()))
        .build(event_loop)
        .unwrap();

    let windows_size = if let Some(monitor) = window.current_monitor() {
        let size = monitor.size();
        (size.width as i32, size.height as i32)
    } else {
        (0, 0)
    };

    window.set_outer_position(position.to_wry_position(
        || event_loop.cursor_position(),
        windows_size,
        translator.size(),
    ));

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

#[derive(Debug, Clone, Default, PartialEq)]
pub enum PositionArg {
    Coordinate(i32, i32),

    #[default]
    Cursor,

    Preset(PresetPosition),
}

/// Preset position
///
/// top-left tl
/// top-center tc
/// top-right tr
/// bottom-left bl
/// bottom-right br
#[derive(Debug, Clone, Default, PartialEq)]
pub enum PresetPosition {
    /// top-left tl
    TopLeft,
    /// top-center tc
    #[default]
    TopRight,
    /// top-right tr
    TopCenter,
    /// center c
    Center,
    /// bottom-left bl
    BottomLeft,
    /// bottom-center bc
    BottomCenter,
    /// bottom-right br
    BottomRight,
}

impl PositionArg {
    pub fn parse(str: &str) -> Result<Self, ParseIntError> {
        if str == "top-left" || str == "tl" {
            return Ok(Self::Preset(PresetPosition::TopLeft));
        }
        if str == "top-center" || str == "tc" {
            return Ok(Self::Preset(PresetPosition::TopCenter));
        }
        if str == "top-right" || str == "tr" {
            return Ok(Self::Preset(PresetPosition::TopRight));
        }

        if str == "bottom-left" || str == "bl" {
            return Ok(Self::Preset(PresetPosition::BottomLeft));
        }
        if str == "bottom-center" || str == "bc" {
            return Ok(Self::Preset(PresetPosition::BottomCenter));
        }
        if str == "bottom-right" || str == "br" {
            return Ok(Self::Preset(PresetPosition::BottomRight));
        }

        let mut pairs = str.split(',');
        if pairs.clone().count() != 2 {
            return Ok(Self::Cursor);
        }

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
    pub fn to_wry_position<T>(
        &self,
        current_cursor_fn: T,
        windows_size: (i32, i32),
        translator_window_size: (u32, u32),
    ) -> PhysicalPosition<i32>
    where
        T: FnOnce() -> Result<PhysicalPosition<f64>, ExternalError>,
    {
        let translator_window_size = (
            translator_window_size.0 as i32,
            translator_window_size.1 as i32,
        );
        match self {
            Self::Coordinate(x, y) => PhysicalPosition::new(*x, *y),
            Self::Cursor => Self::position_map(current_cursor_fn()),
            Self::Preset(p) => match p {
                PresetPosition::TopLeft => PhysicalPosition::new(0, 0),
                PresetPosition::TopCenter => {
                    let (w, _) = windows_size;
                    let (tw, _) = translator_window_size;
                    PhysicalPosition::new((w - tw) / 2, 0)
                }
                PresetPosition::TopRight => {
                    let (w, _) = windows_size;
                    let (tw, _) = translator_window_size;
                    PhysicalPosition::new(w - tw, 0)
                }
                PresetPosition::Center => {
                    let (w, h) = windows_size;
                    let (tw, th) = translator_window_size;
                    PhysicalPosition::new((w + tw) / 2, (h - th) / 2)
                }
                PresetPosition::BottomLeft => {
                    let (_, h) = windows_size;
                    PhysicalPosition::new(0, h)
                }
                PresetPosition::BottomCenter => {
                    let (w, h) = windows_size;
                    let (tw, th) = translator_window_size;
                    PhysicalPosition::new((w - tw) / 2, h - th)
                }
                PresetPosition::BottomRight => {
                    let (w, h) = windows_size;
                    PhysicalPosition::new(w, h)
                }
            },
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
        self.text
            .clone()
            .unwrap_or_else(|| clipboard::read_text().unwrap_or_default())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_arg_parse() {
        let pos = PositionArg::parse("1,2").unwrap();
        assert_eq!(pos, PositionArg::Coordinate(1, 2));

        let pos = PositionArg::parse("1,2,3").unwrap();
        assert_eq!(pos, PositionArg::Cursor);

        let pos = PositionArg::parse("1").unwrap();
        assert_eq!(pos, PositionArg::Cursor);

        let pos = PositionArg::parse("").unwrap();
        assert_eq!(pos, PositionArg::Cursor);

        let pos = PositionArg::parse("top-left").unwrap();
        assert_eq!(pos, PositionArg::Preset(PresetPosition::TopLeft));

        let pos = PositionArg::parse("tl").unwrap();
        assert_eq!(pos, PositionArg::Preset(PresetPosition::TopLeft));

        let pos = PositionArg::parse("top-right").unwrap();
        assert_eq!(pos, PositionArg::Preset(PresetPosition::TopRight));

        let pos = PositionArg::parse("tr").unwrap();
        assert_eq!(pos, PositionArg::Preset(PresetPosition::TopRight));

        let pos = PositionArg::parse("bottom-left").unwrap();
        assert_eq!(pos, PositionArg::Preset(PresetPosition::BottomLeft));

        let pos = PositionArg::parse("bl").unwrap();
        assert_eq!(pos, PositionArg::Preset(PresetPosition::BottomLeft));

        let pos = PositionArg::parse("bottom-right").unwrap();
        assert_eq!(pos, PositionArg::Preset(PresetPosition::BottomRight));

        let pos = PositionArg::parse("br").unwrap();
        assert_eq!(pos, PositionArg::Preset(PresetPosition::BottomRight));

        let pos = PositionArg::parse("br,1").unwrap();
        assert_eq!(pos, PositionArg::Cursor);
    }
}
