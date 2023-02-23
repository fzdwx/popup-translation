#![feature(result_option_inspect)]

use crate::args::{Args, PositionArg};
use crate::translation::Translator;
use clap::Parser;
use std::str::FromStr;
use wry::{
    application::window::WindowId,
    application::{
        accelerator::Accelerator,
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
        global_shortcut::ShortcutManager,
        window::WindowBuilder,
    },
    webview::WebView,
    webview::WebViewBuilder,
};

mod args;
mod clipboard;
mod translation;

#[warn(unused_assignments)]
fn main() -> wry::Result<()> {
    let args: Args = Args::parse();
    let mut _current_webview = None;
    let event_loop = EventLoop::new();

    let mut hotkey_manager = ShortcutManager::new(&event_loop);
    let shortcut_show = Accelerator::from_str(args.show()).unwrap();

    if args.run_once() {
        let (_id, webview) = show(&event_loop, args.translator(), args.text(), args.position());
        _current_webview = Some(webview);
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
                if hotkey_id == shortcut_show.clone().id() {
                    let (_id, webview) =
                        show(event_loop, args.translator(), args.text(), args.position());
                    _current_webview = Some(webview);
                }
            }

            Event::WindowEvent {
                event,  ..
            } => match event {
                WindowEvent::CloseRequested => {
                    _current_webview = None;
                    if args.run_once() {
                        *control_flow = ControlFlow::Exit
                    }
                }
                _ => (),
            },
            _ => (),
        }
    });
}

fn show<T: 'static>(
    event_loop: &EventLoopWindowTarget<T>,
    translator: Translator,
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
        .with_decorations(false)
        .with_resizable(false)
        .with_focused(true)
        .with_visible(false)
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
        .with_user_agent(user_agent_string);

    let web_view = translator.build_webview(webview, text);
    web_view.window().set_visible(true);

    (window_id, web_view)
}
