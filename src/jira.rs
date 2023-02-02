use std::fmt;

pub struct JiraTicket {
    pub key: String,
    pub number: u32,
}

impl JiraTicket {
    pub fn new(key: String, number: u32) -> JiraTicket {
        JiraTicket { key, number }
    }
}

impl fmt::Display for JiraTicket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.key, self.number)
    }
}
