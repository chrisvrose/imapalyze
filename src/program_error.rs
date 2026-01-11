#[derive(Debug)]
pub(crate) enum ProgramError {
    TlsError,
    DotenvError,
    EnvVarError,
    ImapError(imap::Error),
}

impl From<native_tls::Error> for ProgramError {
    fn from(_: native_tls::Error) -> ProgramError {
        ProgramError::TlsError
    }
}

impl From<dotenv::Error> for ProgramError {
    fn from(_: dotenv::Error) -> ProgramError {
        ProgramError::DotenvError
    }
}

impl From<std::env::VarError> for ProgramError {
    fn from(_: std::env::VarError) -> ProgramError {
        ProgramError::EnvVarError
    }
}

impl From<imap::error::Error> for ProgramError {
    fn from(err: imap::error::Error) -> ProgramError {
        ProgramError::ImapError(err)
    }
}
