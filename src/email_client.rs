use crate::domain::SubscriberEmail;

pub struct EmailClient {
  sender: SubscriberEmail
}

impl EmailClient {
  pub async fn send_email(&self, recipient: SubscriberEmail, subject: &str, text_context: &str) -> Result<(), String> {
    todo!()
  }
}