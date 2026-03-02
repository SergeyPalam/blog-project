use reqwest::StatusCode;
use thiserror::Error;

/// Ошибки библиотеки
#[derive(Debug, Error)]
pub enum ClientError {
    /// Ответ сервера, что запись уже существует
    #[error("User already exists: {0}")]
    AlreadyExists(String),
    /// Попытка выполнить неавторизованный запрос
    #[error("User unauthorized: {0}")]
    Unauthorized(String),
    /// Запись не найдена
    #[error("Not found: {0}")]
    NotFound(String),
    /// Неизвестная ошибка сервера
    #[error("Unknown server error: {0}")]
    UnknownServerErr(String),
    /// Внутренняя ошибка сервера
    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl From<reqwest::Error> for ClientError {
    fn from(err: reqwest::Error) -> Self {
        let Some(err_status) = err.status() else {
            return ClientError::InternalError(format!("{err}"));
        };

        match err_status {
            StatusCode::CONFLICT => Self::AlreadyExists(format!("{err}")),
            StatusCode::UNAUTHORIZED => Self::Unauthorized(format!("{err}")),
            StatusCode::NOT_FOUND => Self::NotFound(format!("{err}")),
            _ => Self::UnknownServerErr(format!("{err}")),
        }
    }
}

impl From<tonic::Status> for ClientError {
    fn from(err_status: tonic::Status) -> Self {
        match err_status.code() {
            tonic::Code::AlreadyExists => Self::AlreadyExists(format!("{err_status}")),
            tonic::Code::Unauthenticated => Self::Unauthorized(format!("{err_status}")),
            tonic::Code::NotFound => Self::NotFound(format!("{err_status}")),
            _ => Self::UnknownServerErr(format!("{err_status}")),
        }
    }
}
