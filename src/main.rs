use flexi_logger::FileSpec;
use log::{debug, info};

use crate::{
    program_error::ProgramError,
    tls_imap::{ImapConfig, ImapCredentials},
};

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

    Ok(())
}
