use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::Request;
use rocket::Response;
use serde::Serialize;
use std::error::Error;
use std::fmt;
use std::io::Cursor;
use validator::ValidationErrors;

#[derive(Debug, Serialize)]
pub enum TentechError {
    CannotDecryptToken,
    CannotVerifyPassword,

    ValidationFailed(ValidationErrors),
    TokenExpired,

    DatabaseFailed(String),
    AlreadyActivated,

    CannotSendEmail,

    Unauthorized(String),

    CannotDecodeBase64,

    CannotPutS3Object,
}

#[derive(Debug, Serialize)]
pub struct ErrorJson {
    pub r#type: String,
    pub message: String,
}

impl Into<ErrorJson> for TentechError {
    fn into(self) -> ErrorJson {
        match self {
            TentechError::CannotDecryptToken => ErrorJson {
                r#type: "CannotDecryptToken".to_string(),
                message: format!("{}", self),
            },
            TentechError::CannotVerifyPassword => ErrorJson {
                r#type: "CannotVerifyPassword".to_string(),
                message: format!("{}", self),
            },
            TentechError::ValidationFailed(ref e) => ErrorJson {
                r#type: "ValidationFailed".to_string(),
                message: format!("{}", self),
            },
            TentechError::TokenExpired => ErrorJson {
                r#type: "TokenExpired".to_string(),
                message: format!("{}", self),
            },
            TentechError::DatabaseFailed(ref m) => ErrorJson {
                r#type: "DatabaseFailed".to_string(),
                message: format!("{}", self),
            },
            TentechError::AlreadyActivated => ErrorJson {
                r#type: "AlreadyActivated".to_string(),
                message: format!("{}", self),
            },
            TentechError::CannotSendEmail => ErrorJson {
                r#type: "CannotSendEmail".to_string(),
                message: format!("{}", self),
            },
            TentechError::Unauthorized(ref m) => ErrorJson {
                r#type: "Unauthorized".to_string(),
                message: format!("{}", self),
            },
            TentechError::CannotDecodeBase64 => ErrorJson {
                r#type: "CannotDecodeBase64".to_string(),
                message: format!("{}", self),
            },
            TentechError::CannotPutS3Object => ErrorJson {
                r#type: "CannotPutS3Object".to_string(),
                message: format!("{}", self),
            },
        }
    }
}

impl fmt::Display for TentechError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TentechError::CannotDecryptToken => f.write_str("Cannot decrypt token"),
            TentechError::CannotVerifyPassword => f.write_str("Cannot verify password"),
            TentechError::ValidationFailed(ref e) => e.fmt(f),
            TentechError::TokenExpired => f.write_str("Token expired"),
            TentechError::DatabaseFailed(ref m) => f.write_str(m),
            TentechError::AlreadyActivated => f.write_str("Already activated"),
            TentechError::CannotSendEmail => f.write_str("Cannot send email"),
            TentechError::Unauthorized(ref m) => f.write_str(m),
            TentechError::CannotDecodeBase64 => f.write_str("Cannot decode base64"),
            TentechError::CannotPutS3Object => f.write_str("Cannot put object to s3"),
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
            TentechError::ValidationFailed(_) => Status::BadRequest,
            TentechError::TokenExpired => Status::BadRequest,
            TentechError::DatabaseFailed(_) => Status::Conflict,
            TentechError::AlreadyActivated => Status::Conflict,
            TentechError::CannotSendEmail => Status::UnprocessableEntity,
            TentechError::Unauthorized(_) => Status::Unauthorized,
            TentechError::CannotDecodeBase64 => Status::BadRequest,
            TentechError::CannotPutS3Object => Status::UnprocessableEntity,
        };
        let error: ErrorJson = self.into();
        Response::build()
            .header(ContentType::JSON)
            .status(status)
            .sized_body(Cursor::new(json!(error).to_string()))
            .ok()
    }
}
