use crate::raw::apis::configuration::{ApiKey, Configuration};
use reqwest::Client;

pub const API_URL: &str = "https://api.cloudflareclient.com";
pub const API_VERSION: &str = "v0a1922";
pub const USER_AGENT: &str = "okhttp/3.12.1";
pub const CF_CLIENT_VERSION: &str = "a-6.3-1922";

#[derive(Clone, Debug)]
pub struct Config(pub Configuration);
impl Config {
    pub fn new(auth_token: String) -> Self {
        Self(Configuration {
            bearer_access_token: Some(auth_token),
            ..Self::default().0
        })
    }
}
impl Default for Config {
    fn default() -> Self {
        Self(Configuration {
            base_path: API_URL.into(),
            user_agent: Some(USER_AGENT.into()),
            client: Client::builder()
                .default_headers({
                    use reqwest::header;
                    let mut headers = header::HeaderMap::new();
                    headers.insert(
                        header::USER_AGENT,
                        header::HeaderValue::from_static(USER_AGENT),
                    );
                    headers.insert(
                        header::HeaderName::from_static("cf-client-version"),
                        header::HeaderValue::from_static(CF_CLIENT_VERSION),
                    );
                    headers
                })
                .min_tls_version(reqwest::tls::Version::TLS_1_2)
                .max_tls_version(reqwest::tls::Version::TLS_1_2)
                .build()
                .unwrap(),
            ..Default::default()
        })
    }
}
