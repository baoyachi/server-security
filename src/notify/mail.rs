use async_trait::async_trait;
use duration_str::deserialize_duration;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::transport::smtp::{SMTP_PORT, SUBMISSION_PORT};
use serde::Deserialize;
use std::time::Duration;

use crate::security::ip::{Notify, NotifyMsg};
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};

#[derive(Deserialize, Clone)]
pub struct EmailServer {
    message: EmailMessage,
    credential: Credential,
    smtp: SmtpTrans,
}

#[derive(Deserialize, Clone)]
struct EmailMessage {
    from: String,
    to: String,
}

#[derive(Deserialize, Clone)]
pub struct Credential {
    auth: String,
    secret: String,
}

#[derive(Deserialize, Clone)]
struct SmtpTrans {
    server: String,
    port: u16,
    #[serde(deserialize_with = "deserialize_duration")]
    timeout: Duration,
}

#[async_trait]
impl Notify for EmailServer {
    async fn notify(&self, msg: NotifyMsg) -> anyhow::Result<()> {
        self.send(msg).await
    }
}

impl EmailServer {
    async fn send(&self, msg: NotifyMsg) -> anyhow::Result<()> {
        let message = &self.message;
        let email = Message::builder()
            .from(message.from.parse()?)
            .to(message.to.parse()?)
            .subject(&msg.subject)
            .body(String::from(&msg.body))?;

        let creds = Credentials::new(
            self.credential.auth.to_string(),
            self.credential.secret.to_string(),
        );

        let tls = TlsParameters::builder(self.smtp.server.to_string())
            .dangerous_accept_invalid_certs(true)
            .build()?;

        // Open a remote connection to gmail using STARTTLS
        let mailer: AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&self.smtp.server)?
                .credentials(creds)
                .tls(Tls::Required(tls))
                .port(self.smtp.port)
                // .timeout(Some(Duration::from_secs(5)))
                .build();

        // Send the email
        mailer.send(email).await?;
        Ok(())
    }
}
