use std::rc::Rc;

use anyhow::Result;
use crossterm::event::{Event, EventStream};
use futures_util::stream::StreamExt;
use wasm_bindgen::prelude::*;
use xterm_js_rs::Terminal;

use crate::cli::Cli;
use crate::terminal;
use crate::tui::Tui;

pub trait AppEventHandler {
    /// Handle the terminal event. The returned boolean signals if the [AppMode] should be changed or not.
    fn handle_event(&mut self, event: Event) -> bool;
}

/// The global app struct which handles mode switching and other mode-independent things like resizing
pub struct App {
    terminal: Rc<Terminal>,
    mode: AppMode,
}

impl App {
    pub fn new() -> Result<Self, JsValue> {
        let terminal = Rc::new(terminal::init()?);

        Ok(Self {
            terminal: terminal.clone(),
            mode: AppMode::new(terminal, true),
        })
    }

    pub async fn run(mut self) -> ! {
        let mut stream = EventStream::new(&self.terminal);

        loop {
            let switch_mode = match stream.next().await.unwrap() {
                Ok(event) => self.mode.handle_event(event),
                Err(err) => panic!("Event stream encountered error: {:?}", err),
            };

            if switch_mode {
                match self.mode {
                    AppMode::TUI(_) => self.mode.to_cli(self.terminal.clone()),
                    AppMode::CLI(_) => self.mode.to_tui(self.terminal.clone()),
                }

                // Call init functions
                match self.mode {
                    AppMode::TUI(_) => (),
                    AppMode::CLI(ref mut cli) => cli.init(),
                }
            }
        }
    }
}

/// The app can be either in CLI mode (Only displays markdown text) or in TUI mode where it displays a full text based user interface
pub enum AppMode {
    TUI(Tui),
    CLI(Cli),
}

impl AppMode {
    /// Creates a new AppMode enum. Starts in TUI or CLI mode depending on the provided tui flag
    pub fn new(terminal: Rc<Terminal>, tui: bool) -> Self {
        if tui {
            Self::TUI(Tui::new(terminal))
        } else {
            Self::CLI(Cli::new(terminal))
        }
    }

    /// Switch from cli mode to tui mode
    pub fn to_tui(&mut self, terminal: Rc<Terminal>) {
        *self = Self::TUI(Tui::new(terminal))
    }

    /// Switch from tui mode to cli mode
    pub fn to_cli(&mut self, terminal: Rc<Terminal>) {
        *self = Self::CLI(Cli::new(terminal));
    }
}

impl AppEventHandler for AppMode {
    fn handle_event(&mut self, event: Event) -> bool {
        match self {
            AppMode::TUI(tui) => tui.handle_event(event),
            AppMode::CLI(cli) => cli.handle_event(event),
        }
    }
}
