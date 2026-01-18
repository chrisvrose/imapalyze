use ratatui::{
    text::{Line, Text},
    widgets::{Block, Widget},
};

use crate::App;

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(" Who's sent me these mails? ");
        let bottom_instructions = Line::from(vec![
            " q: ".into(),
            "Quit ".into(),
            " <Arrow>: ".into(),
            "Navigate ".into(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(bottom_instructions.centered());
        block.render(area, buf);
    }
}
