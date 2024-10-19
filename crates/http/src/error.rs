#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Hyper(#[from] hyper::Error),
    #[error(transparent)]
    Argon2(#[from] argon2::password_hash::Error),
    #[error(transparent)]
    Base64Decode(#[from] base64::DecodeError),
    #[error(transparent)]
    Jsonwebtoken(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Pkcs1(#[from] rsa::pkcs1::Error),
    #[error(transparent)]
    Rsa(#[from] rsa::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Time(#[from] std::time::SystemTimeError),
    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),

    #[error("Failed to build server struct")]
    ServerBuilder,

    #[error("unauthorized")]
    Unauthorized,
    #[error("not found")]
    NotFound,
}

pub type Result<T> = std::result::Result<T, Error>;

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::warn!(?self, "error response");
        let status = match self {
            Error::Unauthorized => axum::http::StatusCode::UNAUTHORIZED,
            Error::NotFound => axum::http::StatusCode::NOT_FOUND,

            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}
