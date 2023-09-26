//! Crate global utility functions

pub fn window() -> web_sys::Window {
    web_sys::window().expect("window does not exist")
}
