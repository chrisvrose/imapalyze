use imap::Session;
use std::{
    cmp::min,
    io::{Read, Write},
};

use crate::fetcher::SingleMailMapper;

pub struct ImapSessionBatchedFetcher<T: Read + Write> {
    imap_sess: Session<T>,
    total_count_last: Option<u32>,
}
impl<T: Read + Write> ImapSessionBatchedFetcher<T> {
    pub const BATCH_SIZE: usize = 16000;
    pub fn new(imap_sess: Session<T>) -> Self {
        Self {
            imap_sess,
            total_count_last: None,
        }
    }
    pub fn get_count(&mut self) -> Result<u32, imap::Error> {
        let deets = self.imap_sess.examine("INBOX")?;
        let counts = deets.exists;
        self.total_count_last.insert(counts);
        Ok(counts)
    }
    pub fn total_mails_map<U: SingleMailMapper<V>, V>(&mut self) -> Result<Vec<V>, imap::Error> {
        let counts = self.get_count()?;
        let mut vec = Vec::new();
        for i in (1..counts).step_by(Self::BATCH_SIZE) {
            let range = (i, min(counts, i + Self::BATCH_SIZE as u32 - 1));
            let num_string = format!("{}:{}", range.0, range.1);
            log::info!("Getting emails range {}", num_string);
            let mail_fetch_list = self.imap_sess.fetch(num_string.as_str(), "ALL")?;
            log::info!("Fetched emails range {}", num_string);

            for fetches in mail_fetch_list.iter() {
                vec.push(U::map(fetches));
            }
        }
        Ok(vec)
    }
}
