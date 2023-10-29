use std::cell::{Ref, RefCell};

use crossterm::event::{Event, KeyCode};
use ratatui::{prelude::Backend, Frame};

use super::content::Root;

/// Amount of tabs of this application
const TAB_COUNT: usize = 4;

/**
 * State of the app
 */
pub struct State {
    pub tab: Tab,
    pub row_index: usize,
}

pub struct App {
    state: RefCell<State>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(State {
                tab: Tab::new(TAB_COUNT),
                row_index: 0,
            }),
        }
    }

    pub fn handle_event(&self, event: Event) -> bool {
        match event {
            Event::Key(event) => {
                let mut app_state = self.state.borrow_mut();

                match event.code {
                    KeyCode::Tab => {
                        app_state.tab.next();
                    }
                    KeyCode::BackTab => {
                        app_state.tab.prev();
                    }
                    KeyCode::Up => {
                        // TODO: Very unsafe, need a wrapper like Tab which keeps track of term size and current row
                        app_state.row_index += 1;
                    }
                    KeyCode::Down => {
                        // TODO: See Key UP
                        app_state.row_index -= 1;
                    }
                    KeyCode::Esc => {
                        // Exit tui mode
                        return true;
                    }
                    KeyCode::Char(char) => match char {
                        'q' => return true,
                        _ => (),
                    },
                    _ => (),
                }

                false
            }
            _ => false,
        }
    }

    pub fn render<B: Backend>(&self, frame: &mut Frame<B>) {
        let state = self.state();

        let root = Root::new(&state);

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
    max: usize,
    current: usize,
}

impl Tab {
    fn new(tab_count: usize) -> Self {
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

    pub fn get_current(&self) -> usize {
        self.current
    }
}
