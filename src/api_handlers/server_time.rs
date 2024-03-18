use crate::api_handlers::util::{ApiError, ApiResponse, Message};

pub async fn get_time() -> Result<ApiResponse, ApiError> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| {
            ApiResponse::JsonData(vec![Message {
                message: duration.as_secs().to_string(),
            }])
        })
        .map_err(|_| ApiError::InternalServerError)
}
