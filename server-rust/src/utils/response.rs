use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

pub fn success<T: Serialize>(data: T) -> SuccessResponse<T> {
    SuccessResponse {
        success: true,
        data,
        message: None,
    }
}

pub fn success_with_message<T: Serialize>(data: T, msg: &str) -> SuccessResponse<T> {
    SuccessResponse {
        success: true,
        data,
        message: Some(msg.to_string()),
    }
}
