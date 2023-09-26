use std::cell::{Ref, RefCell};

use crossterm::event::{Event, KeyCode};
use ratatui::{prelude::Backend, Frame};

use crate::demo::{AppContext, Root};

/// Amount of tabs of this application
const TAB_COUNT: u8 = 4;

/**
 * State of the app
 */
pub struct State {
    pub current_tab: Tab,
    pub row_index: u16,
}

pub struct App {
    state: RefCell<State>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(State {
                current_tab: Tab::new(TAB_COUNT),
                row_index: 0,
            }),
        }
    }

    pub fn handle_event(&self, event: Event) {
        match event {
            Event::Key(event) => {
                let mut app_state = self.state.borrow_mut();

                match event.code {
                    KeyCode::Tab => {
                        app_state.current_tab.next();
                    }
                    KeyCode::BackTab => {
                        app_state.current_tab.prev();
                    }
                    KeyCode::Up => {
                        // TODO: Very unsafe, need a wrapper like Tab which keeps track of term size and current row
                        app_state.row_index += 1;
                    }
                    KeyCode::Down => {
                        // TODO: See Key UP
                        app_state.row_index -= 1;
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    pub fn render<B: Backend>(&self, frame: &mut Frame<B>) {
        // TODO: Very much WIP, currently just renders the ratatui demo
        let state = self.state();

        let context = AppContext {
            tab_index: state.current_tab.get_current().into(),
            row_index: state.row_index.into(),
        };

        let root = Root::new(&context);

        frame.render_widget(root, frame.size())
    }

    fn state(&self) -> Ref<State> {
        self.state.borrow()
    }
}

/**
 * zero-based tab navigation
 */
pub struct Tab {
    max: u8,
    current: u8,
}

impl Tab {
    fn new(tab_count: u8) -> Self {
        assert!(tab_count >= 1, "Cannot have less than one tab");

        Self {
            max: tab_count - 1,
            current: 0,
        }
    }

    fn next(&mut self) {
        if self.current == self.max {
            self.current = 0;
        } else {
            self.current += 1;
        }
    }

    fn prev(&mut self) {
        if self.current == 0 {
            self.current = self.max;
        } else {
            self.current -= 1;
        }
    }

    pub fn get_current(&self) -> u8 {
        self.current
    }
}
