mod errors;

use headers_core::Header;
use log::error;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::multipart::{Form, Part};
use reqwest::Body;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio_util::codec::{BytesCodec, FramedRead};

pub use errors::*;
use leptos_cloud_common::config::CloudConfig;
use leptos_cloud_common::net::{
    AppMeta, CheckAvailabilityResponse, LogRequest, LogResponse, LoginResponse, NewAppRequest,
    NewTeamRequest, SetTeamNameRequest, SuccessResponse, Team,
};

const BASE_URL: Option<&str> = option_env!("LEPTOS_CLOUD_API_URL");
const DEFAULT_BASE_URL: &str = "https://leptos.cloud/api/v1/";

#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
    api_key: String,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
        }
    }

    pub async fn teams(self) -> Result<Vec<Team>, ReqwestJsonError> {
        let teams = self.get("teams").send().await?;

        Ok(teams)
    }

    pub async fn new_app(self, app_slug: &str, team_slug: &str) -> Result<bool, ReqwestJsonError> {
        let CheckAvailabilityResponse { available } = self
            .post("apps/new")
            .json(&NewAppRequest {
                app_slug: app_slug.to_string(),
                team_slug: team_slug.to_string(),
            })?
            .send()
            .await?;

        Ok(available)
    }

    pub async fn new_team(self, team_slug: &str) -> Result<bool, ReqwestJsonError> {
        let CheckAvailabilityResponse { available } = self
            .post("teams/new")
            .json(&NewTeamRequest {
                team_slug: team_slug.to_string(),
            })?
            .send()
            .await?;

        Ok(available)
    }

    pub async fn set_team_name(
        self,
        team_slug: &str,
        team_name: &str,
    ) -> Result<(), ReqwestJsonError> {
        let _: SuccessResponse = self
            .post("teams/name")
            .json(&SetTeamNameRequest {
                team_slug: team_slug.to_string(),
                team_name: team_name.to_string(),
            })?
            .send()
            .await?;

        Ok(())
    }

    pub async fn login(self) -> Result<LoginResponse, ReqwestJsonError> {
        Ok(self.post("login").json(())?.send().await?)
    }

    pub async fn upload_file(
        self,
        app_slug: impl AsRef<str>,
        path: impl AsRef<Path>,
    ) -> Result<(), UploadFileError> {
        let file = tokio::fs::File::open(path.as_ref()).await?;
        let read_stream = FramedRead::new(file, BytesCodec::new());

        let stream_part = Part::stream(Body::wrap_stream(read_stream))
            .file_name(path.as_ref().to_string_lossy().to_string());
        let form = Form::new().part("file", stream_part);

        let _: SuccessResponse = self
            .post("apps/upload-file")
            .multipart(form)
            .header(
                AppMeta::name(),
                AppMeta {
                    app_slug: app_slug.as_ref().to_string(),
                }
                .to_string_value(),
            )
            .send()
            .await?;

        Ok(())
    }

    pub async fn upload_done(self, config: &CloudConfig) -> Result<(), ReqwestJsonError> {
        let _: SuccessResponse = self.post("apps/upload-done").json(config)?.send().await?;

        Ok(())
    }

    pub async fn log(self, name: &str) -> Result<String, ReqwestJsonError> {
        let res: LogResponse = self
            .post("log")
            .json(&LogRequest {
                name: name.to_string(),
            })?
            .send()
            .await?;

        Ok(res.log)
    }

    pub fn post(self, route: &str) -> ClientBuilder {
        let url = Self::build_route(route);

        ClientBuilder(self.client.post(url)).auth_header(&self.api_key)
    }

    pub fn get(self, route: &str) -> ClientBuilder {
        let url = Self::build_route(route);

        ClientBuilder(self.client.get(url)).auth_header(&self.api_key)
    }

    fn build_route(route: &str) -> String {
        let base_url = std::env::var("LEPTOS_CLOUD_API_URL")
            .unwrap_or(BASE_URL.unwrap_or(DEFAULT_BASE_URL).to_string());
        format!("{base_url}{route}")
    }
}

pub struct ClientBuilder(reqwest::RequestBuilder);

impl ClientBuilder {
    pub fn auth_header(self, api_key: &str) -> Self {
        Self(
            self.0
                .header("Authorization", format!("Bearer {}", api_key)),
        )
    }

    pub fn body<T: Into<reqwest::Body>>(self, body: T) -> Self {
        Self(self.0.body(body))
    }

    pub fn multipart(self, form: Form) -> Self {
        Self(self.0.multipart(form))
    }

    pub fn json<Body: Serialize>(self, json: Body) -> Result<ClientBuilder, serde_json::Error> {
        let json = serde_json::to_string(&json)?;

        Ok(Self(
            self.0.header("Content-Type", "application/json").body(json),
        ))
    }

    pub fn header<K, V>(self, key: K, value: V) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        Self(self.0.header(key, value))
    }

    pub async fn send<Resp>(self) -> Result<Resp, reqwest::Error>
    where
        for<'de> Resp: Deserialize<'de>,
    {
        let res = self.0.send().await?;

        match res.error_for_status_ref() {
            Ok(_) => Ok(res.json::<Resp>().await?),
            Err(err) => {
                let err_text = res.text().await?;
                error!("Received error:\n{err_text:#?}");

                Err(err)
            }
        }
    }
}
