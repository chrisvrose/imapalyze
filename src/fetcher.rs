use imap::types::{Fetch, Flag};

pub trait SingleMailMapper<T> {
    fn map(mail: &Fetch) -> T;
}

pub struct SingleMailFromAndFlagFetcher;

impl SingleMailMapper<(bool, Vec<(String, String)>)> for SingleMailFromAndFlagFetcher {
    fn map(mail: &Fetch) -> (bool, Vec<(String, String)>) {
        let from_addresses = mail
            .envelope()
            .and_then(|env| env.from.as_ref())
            .map(|items| {
                items.iter().map(|item| {
                    let address_str = from_bytes_to_string(item.mailbox);
                    let host_str = from_bytes_to_string(item.host);

                    (address_str, host_str)
                })
            })
            .map(|e| e.collect())
            .unwrap_or(Vec::default());
        let is_read = mail.flags().iter().find(|e| **e == Flag::Seen).is_some();
        (is_read, from_addresses)
    }
}

fn from_bytes_to_string(value: Option<&[u8]>) -> String {
    value
        .map(|name| {
            String::from_utf8(Vec::from(name))
                .ok()
                .expect("parse failed - not utf8?")
        })
        .unwrap_or(String::new())
}
