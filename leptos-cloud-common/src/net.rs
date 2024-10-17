use headers_core::{Header, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginResponse {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Team {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckNameRequest {
    pub team_id: Option<i64>,
    pub app_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckNameResponse {
    pub available: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogResponse {
    pub log: String,
}

#[derive(Debug, Clone)]
pub struct AppMeta {
    pub name: String,
    pub team_id: Option<i64>,
}

static NAME: HeaderName = HeaderName::from_static("app-meta");

impl AppMeta {
    pub fn to_string_value(&self) -> String {
        let team_id = if let Some(team_id) = self.team_id {
            team_id.to_string()
        } else {
            "".to_string()
        };

        format!("{},{}", team_id, self.name)
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
            .map(|v| {
                v.split_once(',').map(|(team_id, name)| Self {
                    team_id: team_id.parse().ok(),
                    name: name.to_string(),
                })
            })
            .flatten()
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
