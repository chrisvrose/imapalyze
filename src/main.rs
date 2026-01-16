use std::cmp::min;

use flexi_logger::FileSpec;
use log::{debug, info};

use crate::{
    fetcher::{MailStatFromFetcher, SingleMailFromFetcher},
    program_error::ProgramError,
    tls_imap::{ImapConfig, ImapCredentials},
};

mod fetcher;
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

    let mut imap_sess = imap_client.login(email, pwd).map_err(|(e, _)| e)?;

    debug!("Imap session obtained");

    let deets = imap_sess.examine("INBOX")?;

    let counts = deets.exists;

    info!("Found {} emails", counts);

    for x in (1..counts).step_by(1000) {
        let range = (x, min(x + 1000 - 1, counts));
        let num_string = format!("{}:{}", range.0, range.1);
        println!("{}", num_string);
        let last_mail_deets = imap_sess.fetch(num_string.as_str(), "ALL")?;

        assert!(last_mail_deets.len() <= 1000);
        // let last_mail = last_mail_deets
        // .get(0)
        // .ok_or(ProgramError::MailError("Expected atleast one mail"))?;
        for fetched_mail in last_mail_deets.iter() {
            let froms = SingleMailFromFetcher::fetch(fetched_mail);
            println!("DEETS {:?}", froms);
        }
    }
    Ok(())
}
