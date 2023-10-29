//! The terminal user interface

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal as RatatuiTerminal;
use wasm_bindgen::{prelude::Closure, JsCast};
use xterm_js_rs::{crossterm::XtermJsCrosstermBackend, Terminal};

use crate::{app::AppEventHandler, util::window};

mod app;
mod content;

use app::App;

type ShareableTerminal = Rc<RefCell<RatatuiTerminal<CrosstermBackend<XtermJsCrosstermBackend>>>>;

/// The terminal user interface application
pub struct Tui {
    /// Render loop shutdown flag
    shutdown: Rc<Cell<bool>>,
    terminal: ShareableTerminal,
    app: Rc<App>,
}

impl Tui {
    pub fn new(terminal: Rc<Terminal>) -> Self {
        let app = Rc::new(App::new());

        let shutdown = Rc::new(Cell::new(false));

        let mut backend: CrosstermBackend<XtermJsCrosstermBackend> =
            CrosstermBackend::new(terminal);

        execute!(&mut backend, EnterAlternateScreen, EnableMouseCapture)
            .expect("Failed to configure crossterm backend");

        let ratatui = Rc::new(RefCell::new(
            RatatuiTerminal::new(backend).expect("Failed to create ratatui terminal"),
        ));

        start_render_loop(ratatui.clone(), shutdown.clone(), app.clone());

        Self {
            app,
            terminal: ratatui,
            shutdown,
        }
    }
}

impl AppEventHandler for Tui {
    fn handle_event(&mut self, event: crossterm::event::Event) -> bool {
        self.app.handle_event(event)
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        // Exit render loop as well
        self.shutdown.set(true);

        let mut terminal = self.terminal.borrow_mut();

        // Clean up the terminal for CLI mode
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .expect("Failed to configure crossterm backend");

        terminal.clear().unwrap();
    }
}

fn start_render_loop(terminal: ShareableTerminal, shutdown: Rc<Cell<bool>>, app: Rc<App>) {
    let inner = Rc::new(RefCell::new(None));
    let outer = inner.clone();

    *outer.borrow_mut() = Some(Closure::new(move || {
        if shutdown.get() {
            // Take the option to stop the rendering loop
            inner.borrow_mut().take();

            return;
        }

        terminal
            .borrow_mut()
            .draw(|frame| app.render(frame))
            .unwrap();

        request_animation_frame(inner.borrow().as_ref().unwrap());
    }));

    request_animation_frame(outer.borrow().as_ref().unwrap());
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
