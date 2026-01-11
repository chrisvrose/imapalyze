use crate::program_error::ProgramError;

pub struct ImapConfig {
    pub domain: String,
    pub port: u16,
}
impl ImapConfig {
    pub fn from_env() -> Result<ImapConfig, ProgramError> {
        let domain = std::env::var("EMAIL_IMAP_DOMAIN")?;
        let port = std::env::var("EMAIL_IMAP_PORT")?
            .parse()
            .map_err(|_| ProgramError::EnvVarError)?;

        Ok(ImapConfig { domain, port })
    }
}

#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub struct ImapCredentials {
    pub email: String,
    pub pwd: String,
}

impl ImapCredentials {
    pub fn from_env() -> Result<ImapCredentials, ProgramError> {
        let email = std::env::var("EMAIL_LOGIN")?;
        let pwd = std::env::var("EMAIL_PASSWORD")?;

        Ok(ImapCredentials { email, pwd })
    }
}
