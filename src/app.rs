use std::time::Duration;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyEvent, KeyEventKind},
    text::Line,
    widgets::{Block, Widget},
};

use crate::model::mail::MailData;

pub struct App {
    pub exit: bool,
    pub email_store: Option<Vec<MailData>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            email_store: None,
        }
    }
    pub fn run(&mut self, term: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            term.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    pub fn handle_events(&mut self) -> std::io::Result<()> {
        if let true = event::poll(Duration::from_secs(1))? {
            match event::read()? {
                event::Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            }
        }
        Ok(())
    }
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            event::KeyCode::Esc | event::KeyCode::Char('q') => self.mark_exit(),
            _ => {}
        }
    }
    pub fn mark_exit(&mut self) {
        self.exit = true;
    }
}

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
