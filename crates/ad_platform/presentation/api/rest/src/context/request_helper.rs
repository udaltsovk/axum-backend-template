use axum::{
    Json,
    extract::{FromRequest, Request, rejection::JsonRejection},
};
use garde::Validate;
use serde::de::DeserializeOwned;

use crate::context::{errors::AppError, validate::ValidatedRequest};

impl<T, S> FromRequest<S> for ValidatedRequest<T>
where
    T: DeserializeOwned + Validate<Context: Default>,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = AppError;

    async fn from_request(
        req: Request,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedRequest(value))
    }
}
