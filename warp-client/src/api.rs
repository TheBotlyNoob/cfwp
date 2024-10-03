use crate::config::Config;
use base64::Engine;
use time::format_description::well_known::Rfc3339;
use warp_raw_api::{apis::default_api as a, apis::Error as ApiError, models as m};

pub async fn register(
    config: &Config,
    pub_key: &[u8; 32],
    device_model: impl Into<String>,
) -> Result<m::Register200Response, ApiError<a::RegisterError>> {
    a::register(
        &config.0,
        crate::config::API_VERSION,
        Some(m::RegisterRequest {
            fcm_token: String::new(),
            install_id: String::new(),
            key: base64::engine::general_purpose::STANDARD.encode(pub_key),
            locale: "en_US".into(),
            model: device_model.into(),
            tos: time::OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
            r#type: "Android".into(),
        }),
    )
    .await
}
