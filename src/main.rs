use std::{collections::HashMap, time::Duration};

use flexi_logger::FileSpec;
use log::debug;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyEvent, KeyEventKind},
    widgets::Widget,
};

use crate::{
    fetcher::SingleMailFromAndFlagFetcher,
    imap_session_fetcher::ImapSessionBatchedFetcher,
    program_error::ProgramError,
    tls_imap::{ImapConfig, ImapCredentials},
};
mod fetcher;
mod imap_session_fetcher;
mod program_error;
mod tls_imap;
mod window;

fn main() -> Result<(), ProgramError> {
    flexi_logger::Logger::try_with_env_or_str("debug")
        .expect("Unexpected config")
        .log_to_file(FileSpec::default().directory("logs"))
        .start()
        .ok();

    dotenv::dotenv()?;
    ratatui::run(|term| App::new().run(term))?;
    Ok(())
}

pub struct App {
    exit: bool,
}
impl App {
    pub fn new() -> Self {
        Self { exit: false }
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
fn test() -> Result<(), ProgramError> {
    let ImapConfig { domain, port } = ImapConfig::from_env()?;

    let tls_conn = native_tls::TlsConnector::builder()
        .min_protocol_version(Some(native_tls::Protocol::Tlsv12))
        .build()?;
    debug!("TLS obtained");

    let imap_client = imap::connect((domain.as_str(), port), domain.as_str(), &tls_conn)?;

    let ImapCredentials { email, pwd } = ImapCredentials::from_env()?;

    let imap_sess = imap_client.login(email, pwd).map_err(|(e, _)| e)?;
    let mut imap_sess_handler = ImapSessionBatchedFetcher::new(imap_sess);

    let all_mails = imap_sess_handler.total_mails_map::<SingleMailFromAndFlagFetcher, _>()?;

    let mut sender_count = HashMap::<String, u32>::new();
    all_mails
        .iter()
        .filter(|(is_seen, _)| !is_seen)
        .for_each(|(_, senders)| {
            for (_mailbox, host) in senders.iter() {
                let new_count = sender_count.get(host).map_or(0, |x| x.clone()) + 1;
                sender_count.insert(host.clone(), new_count);
            }
        });

    // Build a Vec<(domain, count)> sorted by count descending
    let mut sorted_senders: Vec<(String, u32)> =
        sender_count.iter().map(|(k, v)| (k.clone(), *v)).collect();
    sorted_senders.sort_by(|a, b| b.1.cmp(&a.1));

    println!(
        "Total unread reference: {}",
        sorted_senders.iter().fold(0, |x, (_, c)| { x + c })
    );
    let slice = sorted_senders.iter().take(10);
    // Print full (domain, count) pairs in descending order
    println!("Senders sorted by count: {:?}", slice.collect::<Vec<_>>());

    Ok(())
}
