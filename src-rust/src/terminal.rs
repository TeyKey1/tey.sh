use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::UiEvent;
use xterm_js_rs::xterm::Terminal;
use xterm_js_rs::{addons::fit::FitAddon, TerminalOptions, Theme};

use crate::util::window;

/// Initializes the xtermjs terminal in the html element with id "terminal"
///
/// The terminal is responsive by default using the fit addon which is called on each window resize event
pub fn init() -> Result<Terminal, JsValue> {
    let terminal = Terminal::new(
        TerminalOptions::new()
            .with_cursor_width(10)
            .with_font_size(18)
            .with_font_family("'JetBrains Mono', monospace")
            .with_draw_bold_text_in_bright_colors(true)
            .with_right_click_selects_word(true)
            .with_theme(&terminal_theme())
            .with_tab_stop_width(2)
            .with_convert_eol(true),
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

/// Constructs the color theme of the terminal
///
/// The theme is based on the marine dark theme by ProDeSquare (https://github.com/ProDeSquare) with some color adjustments to match the overall website theme
/// Color theme taken from https://github.com/ProDeSquare/alacritty-colorschemes/blob/master/themes/marine_dark.yaml Apache License 2.0 (https://github.com/ProDeSquare/alacritty-colorschemes/blob/master/LICENSE)
fn terminal_theme() -> Theme {
    let theme = Theme::new();

    theme
        .with_background("#080808")
        .with_foreground("#FFFFFF")
        // Colors
        .with_black("#002221")
        .with_red("#EA3431")
        .with_green("#00B6B6")
        .with_yellow("#E7BF00")
        .with_blue("#4894FD")
        .with_magenta("#E01DCA")
        .with_cyan("#1AB2AD")
        .with_white("#FFFFFF")
        // Bright colors
        .with_bright_black("#006562")
        .with_bright_red("#EA3431")
        .with_bright_green("#00B6B6")
        .with_bright_yellow("#E7BF00")
        .with_bright_blue("#4894FD")
        .with_bright_magenta("#E01DCA")
        .with_bright_cyan("#1AB2AD")
        .with_bright_white("#FFFFFF");

    theme
}
