#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("{message}")]
    NotFound { message: String },

    #[error("invalid money value")]
    InvalidMoneyValue,

    #[error("internal error: {message}")]
    InternalError { message: String },

    #[error("{message}")]
    OrderDomainError { message: String },
}
