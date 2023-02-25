use crate::clipboard;
use crate::translation::{get_translator, Translator};
use clap::{arg, Parser};
use std::num::ParseIntError;
use wry::{application::dpi::PhysicalPosition, application::error::ExternalError};

/// Popup translation
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Text to be translated, if is None, then is daemon mode.
    text: Option<String>,

    /// Platform to be used, available platforms are: bing, dictcn, youdao, youglish, ai(openai gpt)
    ///
    /// --platform=bing
    /// -p ai
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

    /// call the translation api,you need to provide the api key.
    ///
    /// e.g: ai: -p=ai -k=xxx
    ///
    /// --key=xxx
    #[arg(short, long)]
    key: Option<String>,

    /// enable devtool on webview
    #[arg(short, long)]
    debug: bool,
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
        match str {
            "top-left" | "tl" => Ok(Self::Preset(PresetPosition::TopLeft)),
            "top-center" | "tc" => Ok(Self::Preset(PresetPosition::TopCenter)),
            "top-right" | "tr" => Ok(Self::Preset(PresetPosition::TopRight)),
            "center" | "c" => Ok(Self::Preset(PresetPosition::Center)),
            "bottom-left" | "bl" => Ok(Self::Preset(PresetPosition::BottomLeft)),
            "bottom-center" | "bc" => Ok(Self::Preset(PresetPosition::BottomCenter)),
            "bottom-right" | "br" => Ok(Self::Preset(PresetPosition::BottomRight)),
            _ => {
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
        }
    }

    /// convert to wry position
    pub fn to_wry_position<T>(
        &self,
        current_cursor_fn: T,
        windows_size: (i32, i32),
        translator_window_size: (u32, u32),
        start: (i32, i32),
    ) -> PhysicalPosition<i32>
    where
        T: FnOnce() -> Result<PhysicalPosition<f64>, ExternalError>,
    {
        let (tw, th) = (
            translator_window_size.0 as i32,
            translator_window_size.1 as i32,
        );
        let (w, h) = windows_size;
        let (mut x, mut y) = start;

        match self {
            Self::Coordinate(x, y) => PhysicalPosition::new(*x, *y),
            Self::Cursor => Self::position_map(current_cursor_fn()),
            Self::Preset(p) => {
                match p {
                    PresetPosition::TopLeft => {}
                    PresetPosition::TopCenter => {
                        x += (w - tw) / 2;
                    }
                    PresetPosition::TopRight => {
                        x += w - tw;
                    }
                    PresetPosition::Center => {
                        x += (w - tw) / 2;
                        y += (h - th) / 2;
                    }
                    PresetPosition::BottomLeft => {
                        y += h;
                    }
                    PresetPosition::BottomCenter => {
                        x += (w - tw) / 2;
                        y += h - th;
                    }
                    PresetPosition::BottomRight => {
                        x += w;
                        y += h;
                    }
                };

                PhysicalPosition::new(x, y)
            }
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

    pub fn translator(&self) -> Translator {
        match get_translator(self.platform.clone(), self.key.clone()) {
            Ok(t) => t,
            Err(err) => {
                panic!("Failed to get translator: {}", err)
            }
        }
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

    /// enable debug ?
    pub fn dev(&self) -> bool {
        self.debug
    }
}

pub fn ua() -> String {
    #[cfg(target_os = "macos")]
        let user_agent_string = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.1 Safari/605.1.15";
    #[cfg(target_os = "windows")]
        let user_agent_string = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36";
    #[cfg(target_os = "linux")]
        let user_agent_string = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36";
    user_agent_string.to_owned()
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

        let pos = PositionArg::parse("br,1,2").unwrap();
        assert_eq!(pos, PositionArg::Cursor);

        let pos = PositionArg::parse("br,1,2,3").unwrap();
        assert_eq!(pos, PositionArg::Cursor);
    }
}
