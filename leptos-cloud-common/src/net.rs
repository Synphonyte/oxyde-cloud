use headers_core::{Header, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginResponse {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Team {
    pub slug: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewAppRequest {
    pub team_slug: String,
    pub app_slug: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewTeamRequest {
    pub team_slug: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckAvailabilityResponse {
    pub available: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetTeamNameRequest {
    pub team_slug: String,
    pub team_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogResponse {
    pub log: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppMeta {
    pub app_slug: String,
}

static NAME: HeaderName = HeaderName::from_static("app-meta");

impl AppMeta {
    pub fn to_string_value(&self) -> String {
        self.app_slug.clone()
    }
}

impl Header for AppMeta {
    fn name() -> &'static HeaderName {
        &NAME
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers_core::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .next()
            .and_then(|v| v.to_str().ok())
            .map(|v| Self {
                app_slug: v.to_string(),
            })
            .ok_or_else(headers_core::Error::invalid)
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        values.extend([
            HeaderValue::from_str(&self.to_string_value()).expect("invalid header value")
        ]);
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DeploymentStatus {
    Pending = 0,
    Success = 1,
    Failure = 2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuccessResponse {
    pub success: bool,
}

impl Default for SuccessResponse {
    fn default() -> Self {
        Self { success: true }
    }
}
