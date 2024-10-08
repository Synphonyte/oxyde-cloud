mod errors;

use log::error;
use reqwest::multipart::{Form, Part};
use reqwest::Body;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio_util::codec::{BytesCodec, FramedRead};

pub use errors::*;
use leptos_cloud_common::config::CloudConfig;
use leptos_cloud_common::net::{
    CheckNameRequest, CheckNameResponse, LogRequest, LogResponse, LoginResponse,
};

const BASE_URL: &str = "http://localhost:3000/api/v1/";
// const BASE_URL: &str = "https://leptos.cloud/api/v1/";

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

    pub async fn check_name(self, name: &str) -> Result<bool, ReqwestJsonError> {
        let res: CheckNameResponse = self
            .post("check-name")
            .json(&CheckNameRequest {
                name: name.to_string(),
            })?
            .send()
            .await?;

        Ok(res.available)
    }

    pub async fn login(self) -> Result<LoginResponse, ReqwestJsonError> {
        Ok(self.post("login").json(())?.send().await?)
    }

    pub async fn upload_file(self, path: impl AsRef<Path>) -> Result<(), UploadFileError> {
        let file = tokio::fs::File::open(path.as_ref()).await?;
        let read_stream = FramedRead::new(file, BytesCodec::new());

        let stream_part = Part::stream(Body::wrap_stream(read_stream))
            .file_name(path.as_ref().to_string_lossy().to_string());
        let form = Form::new().part("file", stream_part);

        Ok(self.post("upload-file").multipart(form).send().await?)
    }

    // TODO : send config?
    pub async fn upload_done(self, config: &CloudConfig) -> Result<(), ReqwestJsonError> {
        Ok(self.post("upload-done").json(config)?.send().await?)
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
        let url = format!("{BASE_URL}{route}");

        ClientBuilder(self.client.post(url)).auth_header(&self.api_key)
    }
}

pub struct ClientBuilder(reqwest::RequestBuilder);

impl ClientBuilder {
    pub fn auth_header(self, api_key: &str) -> ClientBuilder {
        Self(
            self.0
                .header("Authorization", format!("Bearer {}", api_key)),
        )
    }

    pub fn body<T: Into<reqwest::Body>>(self, body: T) -> ClientBuilder {
        Self(self.0.body(body))
    }

    pub fn multipart(self, form: Form) -> ClientBuilder {
        Self(self.0.multipart(form))
    }

    pub fn json<Body: Serialize>(self, json: Body) -> Result<ClientBuilder, serde_json::Error> {
        let json = serde_json::to_string(&json)?;

        Ok(Self(
            self.0.header("Content-Type", "application/json").body(json),
        ))
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
