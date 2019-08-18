use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::extension::ClientId;
use lettre::smtp::ConnectionReuseParameters;
use lettre::{EmailAddress, Envelope, SendableEmail, SmtpClient, Transport};
use lettre_email::EmailBuilder;
use lettre_email::{mime::TEXT_PLAIN, Email};
use std::env;
use std::path::Path;

pub struct SendError;

pub fn send_activation_email(
    email: &String,
    nickname: &String,
    token: &String,
) -> Option<SendError> {
    let smtp_server = "smtp.gmail.com";
    let smtp_username = "haruan2394@gmail.com";
    let smtp_password = "ahsubgwbgtjxjtqh";

    let email = Email::builder()
        .to((email, nickname))
        .from(smtp_username)
        .subject("Hi, access to this link to activate your account.")
        .text(format!("{}{}", "https://tentech.me/validate?token=", token))
        .build()
        .unwrap();

    let mut mailer = SmtpClient::new_simple(smtp_server)
        .unwrap()
        // Set the name sent during EHLO/HELO, default is `localhost`
        .hello_name(ClientId::Domain(smtp_server.to_string()))
        // Add credentials for authentication
        .credentials(Credentials::new(
            smtp_username.to_string(),
            smtp_password.to_string(),
        ))
        // Enable SMTPUTF8 if the server supports it
        .smtp_utf8(true)
        // Configure expected authentication mechanism
        .authentication_mechanism(Mechanism::Plain)
        // Enable connection reuse
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .transport();

    mailer
        .send(email.into())
        .map_err(|err| {
            println!("Could not send email: {:?}", err);
            SendError {}
        })
        .err()
}
