use ansi_to_tui::IntoText;
use ratatui::{prelude::*, widgets::*};

use crate::{
    content::MD_ABOUT_RENDERED_TUI,
    tui::content::{layout, RgbSwatch, THEME},
};

#[derive(Debug)]
pub struct AboutTab {
    selected_row: usize,
}

impl AboutTab {
    pub fn new(selected_row: usize) -> Self {
        Self { selected_row }
    }
}

impl Widget for AboutTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        RgbSwatch.render(area, buf);
        let area = area.inner(&Margin {
            vertical: 1,
            horizontal: 2,
        });
        Clear.render(area, buf);
        render_about_text(area, buf);
    }
}

fn render_about_text(area: Rect, buf: &mut Buffer) {
    Clear.render(area, buf); // clear out the color swatches
    Block::new().style(THEME.content).render(area, buf);
    let area = area.inner(
        &(Margin {
            vertical: 1,
            horizontal: 2,
        }),
    );

    let (title, content) = &*MD_ABOUT_RENDERED_TUI;

    let text = format!("\n{}", content)
        .into_text()
        .expect("Failed to convert ansi escapes to tui text");

    Paragraph::new(text)
        .style(THEME.description)
        .block(
            Block::new()
                .title(format!(" {} ", title))
                .title_alignment(Alignment::Center)
                .borders(Borders::TOP)
                .border_style(THEME.description_title)
                .padding(Padding::new(0, 0, 0, 0)),
        )
        .wrap(Wrap { trim: true })
        .scroll((0, 0))
        .render(area, buf);
}
