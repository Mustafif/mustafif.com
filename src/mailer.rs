use rocket::FromForm;
use lettre::{Message, message::header::ContentType};
use chrono::offset::Utc;

#[derive(FromForm)]
pub struct ContactForm{
    name: String, 
    email: String, 
    subject: String, 
    message: String
}


impl ContactForm{
    pub fn create_message(&self) -> Message{
        Message::builder()
        .from("mailer@mokareads.org".parse().unwrap())
        .to("mustafif0929@gmail.com".parse().unwrap())
        .subject(&format!("Mailer: {} ({})", &self.subject,Utc::now().to_rfc2822() ))
        .header(ContentType::TEXT_PLAIN)
        .body(
            format!(r#"
            Name: {}
            Email: {}
            Subject: {}
            Message: {}
            "#, &self.name, &self.email, &self.subject, &self.message)
        ).unwrap()
    }
}