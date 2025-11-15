use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::error::{AppError, Result};

#[derive(Clone)]
pub struct EmailService {
    smtp_username: String,
    smtp_password: String,
    smtp_server: String,
    smtp_port: u16,
    from_email: String,
    from_name: String,
}

impl EmailService {
    pub fn from_env() -> Self {
        Self {
            smtp_username: std::env::var("SMTP_USERNAME")
                .unwrap_or_else(|_| "test@ethereal.email".to_string()),
            smtp_password: std::env::var("SMTP_PASSWORD")
                .unwrap_or_else(|_| "password".to_string()),
            smtp_server: std::env::var("SMTP_SERVER")
                .unwrap_or_else(|_| "smtp.ethereal.email".to_string()),
            smtp_port: std::env::var("SMTP_PORT")
                .unwrap_or_else(|_| "587".to_string())
                .parse()
                .unwrap_or(587),
            from_email: std::env::var("FROM_EMAIL")
                .unwrap_or_else(|_| "noreply@phisherman.test".to_string()),
            from_name: std::env::var("FROM_NAME")
                .unwrap_or_else(|_| "PhisherMan Security".to_string()),
        }
    }

    pub fn send_email(
        &self,
        to_email: &str,
        to_name: Option<&str>,
        subject: &str,
        body: &str,
    ) -> Result<String> {
        let to = if let Some(name) = to_name {
            format!("{} <{}>", name, to_email)
        } else {
            to_email.to_string()
        };

        let email = Message::builder()
            .from(format!("{} <{}>", self.from_name, self.from_email).parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body.to_string())
            .map_err(|e| AppError::Internal(format!("Failed to build email: {}", e)))?;

        let creds = Credentials::new(
            self.smtp_username.clone(),
            self.smtp_password.clone(),
        );

        let mailer = SmtpTransport::relay(&self.smtp_server)
            .map_err(|e| AppError::Internal(format!("Failed to connect to SMTP: {}", e)))?
            .credentials(creds)
            .port(self.smtp_port)
            .build();

        match mailer.send(&email) {
            Ok(response) => {
                tracing::info!("Email sent successfully: {:?}", response);
                Ok(format!("Email sent to {}", to_email))
            }
            Err(e) => {
                tracing::error!("Failed to send email: {}", e);
                Err(AppError::Internal(format!("Failed to send email: {}", e)))
            }
        }
    }
}
