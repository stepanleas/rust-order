use crate::responses::ErrorResponse;
use crate::validation::ValidationFieldError;
use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use anyhow::Error as AnyhowError;
use domain::DomainError;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("{message}")]
    Generic {
        code: StatusCode,
        message: String,
        #[source]
        source: Option<AnyhowError>,
    },

    #[error("Validation error")]
    Validation {
        code: StatusCode,
        message: String,
        errors: Vec<ValidationFieldError>,
    },
}

impl ApiError {
    pub fn internal(err: impl Into<AnyhowError>) -> Self {
        ApiError::Generic {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal server error".to_string(),
            source: Some(err.into()),
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        ApiError::Generic {
            code: StatusCode::BAD_REQUEST,
            message: message.into(),
            source: None,
        }
    }
}

impl From<AnyhowError> for ApiError {
    fn from(err: AnyhowError) -> Self {
        if let Some(domain_err) = err.downcast_ref::<DomainError>() {
            return match domain_err {
                DomainError::OrderDomainError { .. } => ApiError::Generic {
                    code: StatusCode::UNPROCESSABLE_ENTITY,
                    message: domain_err.to_string(),
                    source: Some(err),
                },
                DomainError::InvalidMoneyValue => ApiError::Generic {
                    code: StatusCode::BAD_REQUEST,
                    message: domain_err.to_string(),
                    source: Some(err),
                },
                DomainError::NotFound { .. } => ApiError::Generic {
                    code: StatusCode::NOT_FOUND,
                    message: domain_err.to_string(),
                    source: Some(err),
                },
                DomainError::InternalError { .. } => ApiError::Generic {
                    code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: domain_err.to_string(),
                    source: Some(err),
                },
            };
        }

        ApiError::Generic {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal server error".to_string(),
            source: Some(err),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::Generic {
                code: status_code, ..
            } => *status_code,
            ApiError::Validation {
                code: status_code, ..
            } => *status_code,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::Generic {
                code: status_code,
                message,
                ..
            } => HttpResponse::build(*status_code).json(ErrorResponse {
                code: u16::from(*status_code),
                message: message.clone(),
                errors: None,
            }),

            ApiError::Validation {
                code: status_code,
                message,
                errors,
            } => HttpResponse::build(*status_code).json(ErrorResponse {
                code: u16::from(*status_code),
                message: message.clone(),
                errors: Some(errors.clone()),
            }),
        }
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(errors: ValidationErrors) -> Self {
        let mut result = Vec::new();

        for (field, validation_errors) in errors.field_errors().iter() {
            for validation_error in validation_errors.iter() {
                let message = validation_error
                    .message
                    .clone()
                    .unwrap_or_else(|| "Invalid value".into())
                    .to_string();

                result.push(ValidationFieldError {
                    field: field.to_string(),
                    message,
                });
            }
        }

        ApiError::Validation {
            code: StatusCode::BAD_REQUEST,
            message: "Validation error".to_string(),
            errors: result,
        }
    }
}
