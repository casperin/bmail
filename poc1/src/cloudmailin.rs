use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Incoming {
    pub envelope: Envelope,
    pub headers: Headers,
    // pub html: Option<String>,
    pub plain: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Envelope {
    pub from: String,
    pub to: String,
    // pub recipients: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Headers {
    pub date: String,
    pub subject: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_incoming_mail() {
        let json = include_str!("../tests/payloads/cloudmailin_incoming_1.json");
        let incoming: Incoming = serde_json::from_str(json).unwrap();
        assert_eq!(incoming.plain, "This is a test\n");
        // assert_eq!(incoming.html, None);
        assert_eq!(incoming.envelope.from, "mail@gormcasper.dk");
    }
}
