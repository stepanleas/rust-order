use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("item with id {id} not found")]
    NotFound { id: Uuid },

    #[error("invalid money value")]
    InvalidMoneyValue,

    #[error("internal error: {message}")]
    InternalError { message: String },

    #[error("domain error: {message}")]
    OrderDomainError { message: String },
}
