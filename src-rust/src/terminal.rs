use std::rc::Rc;

use crossterm::{event::EnableMouseCapture, execute, terminal::EnterAlternateScreen};
use ratatui::prelude::CrosstermBackend;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::UiEvent;
use xterm_js_rs::{
    addons::fit::FitAddon, crossterm::XtermJsCrosstermBackend, Terminal, TerminalOptions, Theme,
};

use crate::util::window;

/// Initializes the xtermjs terminal in the html element with id "terminal"
///
/// The terminal is responsive by default using the fit addon which is called on each window resize event
pub fn init() -> Result<Terminal, JsValue> {
    let terminal = Terminal::new(
        TerminalOptions::new()
            .with_cursor_width(10)
            .with_font_size(20)
            .with_font_family("monospace")
            .with_draw_bold_text_in_bright_colors(true)
            .with_right_click_selects_word(true)
            .with_scrollback(0)
            .with_theme(Theme::new().with_background("#080808")),
    );

    let wrapper_element = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("terminal")
        .unwrap();

    terminal.open(wrapper_element.dyn_into()?);
    terminal.focus();

    // Make terminal responsive
    let addon = FitAddon::new();
    terminal.load_addon(addon.clone().dyn_into::<FitAddon>()?.into());
    addon.fit();

    let callback = Closure::wrap(Box::new(move |_: UiEvent| {
        addon.fit();
    }) as Box<dyn FnMut(_)>);

    window()
        .add_event_listener_with_callback("resize", callback.into_js_value().unchecked_ref())
        .expect("Failed to attach resize event listener to window");

    Ok(terminal)
}

/// Init and configure crossterm backend
pub fn init_backend(terminal: Rc<Terminal>) -> CrosstermBackend<XtermJsCrosstermBackend> {
    let mut backend: CrosstermBackend<XtermJsCrosstermBackend> =
        CrosstermBackend::new(terminal.clone());

    execute!(&mut backend, EnterAlternateScreen, EnableMouseCapture)
        .expect("Failed to configure crossterm backend");

    backend
}
