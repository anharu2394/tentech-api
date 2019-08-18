use diesel;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::Request;
use rocket::Response;
use rocket_contrib::json::Json;
use serde::Serialize;
use std::error::Error;
use std::fmt;
use std::io::Cursor;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug, Serialize)]
pub enum TentechError {
    CannotDecryptToken,
    CannotVerifyPassword,

    ValidationFailed(ValidationErrors),
    TokenExpired,

    DatabaseFailed(String),
    AlreadyActivated,

    CannotSendEmail,
}

impl fmt::Display for TentechError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TentechError::CannotDecryptToken => f.write_str("Cannot decrypt token"),
            TentechError::CannotVerifyPassword => f.write_str("Cannot verify password"),
            TentechError::ValidationFailed(ref e) => e.fmt(f),
            TentechError::DatabaseFailed(ref m) => f.write_str(m),
            TentechError::CannotSendEmail => f.write_str("Cannot send email"),
        }
    }
}

impl Error for TentechError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            TentechError::ValidationFailed(ref e) => Some(e),
            _ => None,
        }
    }
}

impl Responder<'static> for TentechError {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        let status = match self {
            TentechError::CannotDecryptToken => Status::Unauthorized,
            TentechError::CannotVerifyPassword => Status::Unauthorized,
            TentechError::ValidationFailed(ref e) => Status::BadRequest,
            TentechError::DatabaseFailed(ref e) => Status::Conflict,
            TentechError::CannotSendEmail => Status::UnprocessableEntity,
        };
        Response::build()
            .header(ContentType::JSON)
            .status(status)
            .sized_body(Cursor::new(json!({ "error": self }).to_string()))
            .ok()
    }
}
