use crate::error::ApiError;
use actix_web::dev::Payload;
use actix_web::web::Json;
use actix_web::{FromRequest, HttpRequest};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::pin::Pin;
use validator::Validate;

#[derive(Debug, Serialize, Clone)]
pub struct ValidationFieldError {
    pub field: String,
    pub message: String,
}

pub struct ValidatedJson<T>(pub T);

impl<T> ValidatedJson<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for ValidatedJson<T> {
    fn from(value: T) -> Self {
        ValidatedJson(value)
    }
}

impl<T> FromRequest for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + 'static,
{
    type Error = ApiError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let fut = Json::<T>::from_request(req, payload);

        Box::pin(async move {
            let json = fut
                .await
                .map_err(|err| ApiError::bad_request(err.to_string()))?;

            json.validate()
                .map(|_| ValidatedJson(json.into_inner()))
                .map_err(ApiError::from)
        })
    }
}
