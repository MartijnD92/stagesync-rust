use crate::types::ErrorMessage;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use actix_web_httpauth::headers::www_authenticate::bearer::Bearer;
use derive_more::Display;
use jsonwebtoken::jwk::AlgorithmParameters;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display("Internal Server Error")]
    InternalServerError,

    #[display("BadRequest: {}", _0)]
    BadRequest(String),

    #[display("Unauthorized")]
    Unauthorized,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => HttpResponse::InternalServerError()
                .json("Internal Server Error, please try again later"),
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => {
                HttpResponse::Unauthorized().json("Unauthorized, insufficient permissions")
            }
        }
    }
}

impl From<actix_web::error::BlockingError> for ServiceError {
    fn from(error: actix_web::error::BlockingError) -> ServiceError {
        log::error!("actix threadpool pool error: {}", error);
        ServiceError::InternalServerError
    }
}

#[derive(Debug, Display)]
pub enum ClientError {
    #[display("authentication")]
    Authentication(actix_web_httpauth::extractors::AuthenticationError<Bearer>),
    #[display("decode")]
    Decode(jsonwebtoken::errors::Error),
    #[display("not_found")]
    NotFound(String),
    #[display("unsupported_algorithm")]
    UnsupportedAlgortithm(AlgorithmParameters),
}

impl ResponseError for ClientError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::Authentication(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: None,
                error_description: None,
                message: "Requires authentication".to_string(),
            }),
            Self::Decode(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(
                    "Authorization header value must follow this format: Bearer access-token"
                        .to_string(),
                ),
                message: "Bad credentials".to_string(),
            }),
            Self::NotFound(msg) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(msg.to_string()),
                message: "Bad credentials".to_string(),
            }),
            Self::UnsupportedAlgortithm(alg) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(format!(
                    "Unsupported encryption algortithm, expected RSA got {:?}",
                    alg
                )),
                message: "Bad credentials".to_string(),
            }),
        }
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}
