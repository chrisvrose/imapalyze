use std::collections::HashMap;

use flexi_logger::FileSpec;
use log::debug;

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

fn main() -> Result<(), ProgramError> {
    flexi_logger::Logger::try_with_env_or_str("debug")
        .expect("Unexpected config")
        .log_to_file(FileSpec::default().directory("logs"))
        .start()
        .ok();

    dotenv::dotenv()?;

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

    println!("all mails {:?}", sender_count);

    Ok(())
}
