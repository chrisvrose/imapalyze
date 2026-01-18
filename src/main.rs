use std::collections::HashMap;

use flexi_logger::FileSpec;
use log::debug;

use crate::{
    app::App,
    fetcher::SingleMailFromAndFlagFetcher,
    imap_session_fetcher::ImapSessionBatchedFetcher,
    model::mail::MailData,
    program_error::ProgramError,
    tls_imap::{ImapConfig, ImapCredentials},
};
mod app;
mod fetcher;
mod imap_session_fetcher;
mod model;
mod program_error;
mod tls_imap;
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
        .filter(|MailData { is_read, .. }| !is_read)
        .for_each(|MailData { from, .. }| {
            for (_mailbox, host) in from.iter() {
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
