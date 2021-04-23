use duration_str::deserialize_duration;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::transport::smtp::{SMTP_PORT, SUBMISSION_PORT};
use serde::Deserialize;
use std::time::Duration;

use lettre::{Message, SmtpTransport, Transport};


#[derive(Deserialize,Clone)]
pub struct EmailServer {
    message: EmailMessage,
    credential: Credential,
    smtp: SmtpTrans,
}

#[derive(Deserialize,Clone)]
struct EmailMessage {
    from: String,
    to: String,
    subject: String,
    body: String,
}

#[derive(Deserialize,Clone)]
pub struct Credential {
    auth: String,
    secret: String,
}

#[derive(Deserialize,Clone)]
struct SmtpTrans {
    server: String,
    port: u16,
    #[serde(deserialize_with = "deserialize_duration")]
    timeout: Duration,
}

impl EmailServer {
    fn send(&self) -> anyhow::Result<()> {
        let message = &self.message;
        let email = Message::builder()
            .from(message.from.parse()?)
            .to(message.to.parse()?)
            .subject(&message.subject)
            .body(String::from(&message.body))?;

        let creds = Credentials::new(
            self.credential.auth.to_string(),
            self.credential.secret.to_string(),
        );

        let tls = TlsParameters::builder(self.smtp.server.to_string())
            .dangerous_accept_invalid_certs(true)
            .build()?;

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay(&self.smtp.server)?
            .credentials(creds)
            .tls(Tls::Required(tls))
            .port(self.smtp.port)
            .timeout(Some(Duration::from_secs(5)))
            .build();

        // Send the email
        mailer.send(&email)?;
        Ok(())
    }
}