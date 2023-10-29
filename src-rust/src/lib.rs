use anyhow::Result;
use wasm_bindgen::prelude::*;

mod app;
mod cli;
mod content;
mod terminal;
mod tui;
mod util;

use app::App;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    // nice wasm panic
    console_error_panic_hook::set_once();

    // wasm logger
    wasm_logger::init(wasm_logger::Config::default());

    // Override colored auto detect of the terminals coloring ability (xtermjs does not set the required variables for autodetect)
    colored::control::set_override(true);

    content::prepare_content();

    // Create and start app
    let app = App::new()?;

    app.run().await;

    Ok(())
}
