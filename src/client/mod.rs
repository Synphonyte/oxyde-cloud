mod responses;

use std::path::Path;
use log::error;
use reqwest::Body;
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use serde_json::Error;
use tokio_util::codec::{BytesCodec, FramedRead};

pub use responses::*;

const BASE_URL: &str = "https://leptos.cloud/api/v1/";

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

    pub async fn login(self) -> Result<LoginResponse, reqwest::Error> {
        self.post("login").body("{}").send().await
    }

    pub async fn upload_file(self, path: impl AsRef<Path>) -> std::io::Result<ClientBuilder> {
        let file = tokio::fs::File::open(path).await?;
        let read_stream = FramedRead::new(file, BytesCodec::new());

        let stream_part = Part::stream(Body::wrap_stream(read_stream)).file_name(path);
        let form = Form::new().part("file", stream_part);

        Ok(self.post("upload-file").multipart(form))
    }

    // TODO : send config?
    pub fn upload_done(self) -> ClientBuilder {
        self.post("upload-done").body("{}")
    }

    pub fn post(self, route: &str) -> ClientBuilder {
        let url = format!("{BASE_URL}{route}");

        ClientBuilder(
            self.client
                .post(url)
                .header("AuthenticationToken", self.api_key),
        )
    }
}

pub struct ClientBuilder(reqwest::RequestBuilder);


impl ClientBuilder {
    pub fn body<T: Into<reqwest::Body>>(self, body: T) -> ClientBuilder {
        Self(self.0.body(body))
    }

    pub fn multipart(self, form: Form) -> ClientBuilder {
        Self(self.0.multipart(form))
    }

    pub fn json<Body: Serialize>(self, json: Body) -> Result<ClientBuilder, serde_json::Error> {
        let json = serde_json::to_string(&json)?;

        Ok(Self(self.0.header("Content-Type", "application/json").body(json)))
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