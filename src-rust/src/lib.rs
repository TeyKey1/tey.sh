use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;
use crossterm::event::EventStream;
use futures_util::stream::StreamExt;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal as RatatuiTerminal;
use util::window;
use wasm_bindgen::prelude::*;
use xterm_js_rs::crossterm::XtermJsCrosstermBackend;

mod app;
mod demo;
mod terminal;
mod util;

use crate::app::App;

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    // nice wasm panic
    console_error_panic_hook::set_once();

    // wasm logger
    wasm_logger::init(wasm_logger::Config::default());

    let terminal = Rc::new(terminal::init()?);

    let backend = terminal::init_backend(terminal.clone());

    let ratatui = RatatuiTerminal::new(backend).expect("Failed to create ratatui terminal");

    let app = Rc::new(App::new());

    log::info!("starting rendering looop");

    start_render_loop(ratatui, app.clone());

    let mut stream = EventStream::new(&*terminal);

    loop {
        match stream.next().await.unwrap() {
            Ok(event) => app.handle_event(event),
            Err(err) => panic!("Event stream encountered error: {:?}", err),
        }
    }
}

fn start_render_loop(
    mut terminal: RatatuiTerminal<CrosstermBackend<XtermJsCrosstermBackend>>,
    app: Rc<App>,
) {
    let inner = Rc::new(RefCell::new(None));
    let outer = inner.clone();

    *outer.borrow_mut() = Some(Closure::new(move || {
        terminal.draw(|frame| app.render(frame)).unwrap();

        request_animation_frame(inner.borrow().as_ref().unwrap());
    }));

    request_animation_frame(outer.borrow().as_ref().unwrap());
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
