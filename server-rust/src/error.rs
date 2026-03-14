use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("验证错误: {0}")]
    ValidationError(String),

    #[error("未找到: {0}")]
    NotFoundError(String),

    #[error("AI 服务错误: {0}")]
    AIServiceError(String),

    #[error("内部错误: {0}")]
    InternalError(String),
}

impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse;

        let error_message = self.to_string();

        HttpResponse::build(self.status_code()).json(serde_json::json!({
            "success": false,
            "error": error_message
        }))
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::ValidationError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AppError::NotFoundError(_) => actix_web::http::StatusCode::NOT_FOUND,
            AppError::AIServiceError(_) => actix_web::http::StatusCode::SERVICE_UNAVAILABLE,
            AppError::InternalError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
