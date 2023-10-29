//! Parses and prepares the markdown content for use in cli and tui modes
use lazy_static::{initialize, lazy_static};

mod render_md;

use render_md::{render_md, split_md_title};

lazy_static! {
    // Raw MD strings
    pub static ref MD_ABOUT: &'static str = include_str!("../../../content/about.md");
    pub static ref MD_HOME: &'static str = include_str!("../../../content/home.md");
    pub static ref MD_PROJECTS: &'static str = include_str!("../../../content/projects.md");
    pub static ref MD_SKILLS: &'static str = include_str!("../../../content/skills.md");
    pub static ref MD_HOME_TUI: &'static str = include_str!("../../../content/home-tui.md");

    // Rendered MD strings for CLI
    pub static ref MD_ABOUT_RENDERED: String = render_md(*MD_ABOUT);
    pub static ref MD_HOME_RENDERED: String = render_md(*MD_HOME);
    pub static ref MD_PROJECTS_RENDERED: String =
        render_md(*MD_PROJECTS);
    pub static ref MD_SKILLS_RENDERED: String =
        render_md(*MD_SKILLS);
    pub static ref MD_HOME_TUI_RENDERED: String =
        render_md(*MD_HOME_TUI);

    // Rendered MD strings for TUI
    pub static ref MD_ABOUT_RENDERED_TUI: (String, String) = split_md_title(*MD_ABOUT);
    pub static ref MD_HOME_RENDERED_TUI: (String, String) = split_md_title(*MD_HOME);
    pub static ref MD_PROJECTS_RENDERED_TUI: (String, String) = split_md_title(*MD_PROJECTS);
    pub static ref MD_SKILLS_RENDERED_TUI: (String, String) = split_md_title(*MD_SKILLS);
}

/// Prepare the markdown content loading/rendering
pub fn prepare_content() {
    initialize(&MD_ABOUT);
    initialize(&MD_HOME);
    initialize(&MD_PROJECTS);
    initialize(&MD_SKILLS);
    initialize(&MD_HOME_TUI);
    initialize(&MD_ABOUT_RENDERED_TUI);
    initialize(&MD_HOME_RENDERED_TUI);
    initialize(&MD_PROJECTS_RENDERED_TUI);
    initialize(&MD_SKILLS_RENDERED_TUI);
}
