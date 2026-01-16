use imap::types::{Fetch, ZeroCopy};

pub trait MailStatFromFetcher<T> {
    fn fetch(mail: &Fetch) -> Vec<T>;
}

pub struct SingleMailFromFetcher;

impl MailStatFromFetcher<String> for SingleMailFromFetcher {
    fn fetch(mail: &Fetch) -> Vec<String> {
        let a = mail
            .envelope()
            .and_then(|env| env.from.as_ref())
            .map(|items| {
                items.iter().map(|item| {
                    let address_str = from_bytes_to_string(item.mailbox)
                        + "@"
                        + from_bytes_to_string(item.host).as_str();
                    address_str
                })
            })
            .expect("Unexpected Missing From address");
        a.collect()
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
