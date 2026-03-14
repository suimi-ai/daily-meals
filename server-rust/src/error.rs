use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use chrono::Utc;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("验证错误: {0}")]
    ValidationError(String),

    #[error("资源未找到: {0}")]
    NotFoundError(String),

    #[error("AI 服务错误: {0}")]
    AIServiceError(String),

    #[error("内部错误: {0}")]
    InternalError(String),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: ErrorDetail,
}

#[derive(Serialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    pub timestamp: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (error_code, status_code) = match self {
            AppError::ValidationError(_) => ("VALIDATION_ERROR", StatusCode::BAD_REQUEST),
            AppError::NotFoundError(_) => ("NOT_FOUND", StatusCode::NOT_FOUND),
            AppError::AIServiceError(_) => ("AI_SERVICE_ERROR", StatusCode::INTERNAL_SERVER_ERROR),
            AppError::InternalError(_) => ("INTERNAL_ERROR", StatusCode::INTERNAL_SERVER_ERROR),
        };

        HttpResponse::build(status_code).json(ErrorResponse {
            success: false,
            error: ErrorDetail {
                code: error_code.to_string(),
                message: self.to_string(),
                timestamp: Utc::now().to_rfc3339(),
            },
        })
    }
}
