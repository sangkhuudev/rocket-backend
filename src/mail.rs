use lettre::{message::{header::ContentType, MessageBuilder}, transport::smtp::response::Response, SmtpTransport};
use lettre::Transport;
use tera::{Context, Tera};

pub struct HtmlMailer {
    pub template_engine: Tera,
    pub smtp_host: String,
    pub smtp_port: String,
    pub smtp_username: String,
    pub smtp_password: String,
}

impl HtmlMailer {
    pub fn send(
        self, 
        to_email: String, 
        template_name: &str, 
        template_context: Context
    ) -> Result<Response, Box<dyn std::error::Error>>{
        let html_body = self.template_engine.render(template_name, &template_context)?;
        
        // Use bulder pattern here to create message
        let message = MessageBuilder::new()
            .subject("Rocket backend digest")
            .from("mailtrap@demomailtrap.com".parse()?)
            .to(to_email.parse()?)
            .header(ContentType::TEXT_HTML)
            .body(html_body)?;

        let mailer = SmtpTransport::from_url(&format!(
            "smtp://{}:{}@{}:{}?tls=required",
            self.smtp_username,
            self.smtp_password,
            self.smtp_host,
            self.smtp_port
            ))
            .unwrap()
            .build();

        mailer.send(&message).map_err(|e| e.into())
    }
}
