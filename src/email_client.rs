//! src/email_client.rs

use crate::domain::SubscriberEmail;
use reqwest::Client;
#[allow(dead_code)]
pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
}

impl EmailClient {
    pub fn new(base_url: String, sender: SubscriberEmail) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            sender,
        }
    }
    pub async fn send_email(
        &self,
        _recipient: SubscriberEmail,
        _subject: &str,
        _text_context: &str,
    ) -> Result<(), String> {
        todo!()
    }
}
