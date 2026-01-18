pub struct MailData {
    pub is_read: bool,
    pub from: Vec<(String, String)>,
}

impl From<(bool, Vec<(String, String)>)> for MailData {
    fn from(value: (bool, Vec<(String, String)>)) -> Self {
        MailData {
            is_read: value.0,
            from: value.1,
        }
    }
}
