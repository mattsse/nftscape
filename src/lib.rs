use crate::error::ApiError;
use log::debug;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{header, IntoUrl, StatusCode, Url};
use serde::de::DeserializeOwned;

use serde::Serialize;
use std::convert::TryInto;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;

pub mod opensea;

#[cfg(feature = "rarible")]
pub mod rarible;

mod error;

#[derive(Clone)]
pub struct ApiClient {
    /// The client that executes the http requests
    client: Arc<reqwest::Client>,

    /// Base url to the endpoint
    base_path: Url,
}

impl ApiClient {
    pub fn new(client: Arc<reqwest::Client>, base_path: Url) -> Self {
        Self { client, base_path }
    }

    pub fn join_url<T: AsRef<str>>(&self, path: T) -> anyhow::Result<Url> {
        Ok(self.base_path.join(path.as_ref())?)
    }

    async fn request_json<T, E>(req: reqwest::RequestBuilder) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
        E: DeserializeOwned + Into<ApiError>,
    {
        debug!("Request: {:?}", req);
        let resp = Self::request::<E>(req).await?.bytes().await?;
        match serde_json::from_slice::<T>(&resp) {
            Ok(resp) => Ok(resp),
            Err(err) => {
                if let Ok(api_err) = serde_json::from_slice::<E>(&resp) {
                    Err(api_err.into().into())
                } else {
                    Err(err.into())
                }
            }
        }
    }

    #[allow(unused)]
    async fn post_json<T, U, B, E>(&self, url: U, body: Option<&B>) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
        U: IntoUrl,
        B: Serialize + ?Sized,
        E: DeserializeOwned + Into<ApiError>,
    {
        let mut req = self.client.post(url).header(
            header::ACCEPT,
            header::HeaderValue::from_static("application/json"),
        );
        if let Some(body) = body {
            req = req.json(body);
        }
        Self::request_json::<_, E>(req).await
    }

    /// Execute the request.
    async fn request<E>(req: reqwest::RequestBuilder) -> anyhow::Result<reqwest::Response>
    where
        E: DeserializeOwned + Into<ApiError>,
    {
        let resp = req.send().await?;

        if resp.status().is_success() {
            Ok(resp)
        } else {
            let status = resp.status();
            if let Ok(err) = resp.json::<E>().await {
                return Err(err.into().into());
            }
            let err = match status {
                StatusCode::UNAUTHORIZED => ApiError::Unauthorized,
                status => ApiError::Other(status.as_u16()),
            };
            Err(err.into())
        }
    }

    /// Convenience method to create a [`ApiClientBuilder`]
    pub fn builder() -> ApiClientBuilder {
        ApiClientBuilder::default()
    }

    pub fn base_path(&self) -> &Url {
        &self.base_path
    }

    pub fn client(&self) -> &Arc<reqwest::Client> {
        &self.client
    }
}

#[derive(Default, Debug, Clone)]
pub struct ApiClientBuilder {
    base_path: Option<Url>,
    user_agent: Option<String>,
    client: Option<Arc<reqwest::Client>>,
    headers: Option<HeaderMap>,
    download_dir: Option<PathBuf>,
}

impl ApiClientBuilder {
    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        self.headers.get_or_insert(HeaderMap::default())
    }

    /// Inserts a `Basic` Authorization header
    pub fn basic_auth(
        mut self,
        username: impl AsRef<u8>,
        password: Option<impl AsRef<u8>>,
    ) -> anyhow::Result<Self> {
        let mut header_value = b"Basic ".to_vec();
        {
            let mut encoder =
                base64::write::EncoderWriter::new(&mut header_value, base64::STANDARD);
            write!(encoder, "{}:", username.as_ref())?;
            if let Some(password) = password {
                write!(encoder, "{}", password.as_ref())?;
            }
        }
        self.headers_mut()
            .insert(header::AUTHORIZATION, header_value.try_into()?);
        Ok(self)
    }

    pub fn user_agent<T: ToString>(mut self, user_agent: T) -> Self {
        self.user_agent = Some(user_agent.to_string());
        self
    }

    pub fn client(mut self, client: Arc<reqwest::Client>) -> Self {
        self.client = Some(client);
        self
    }

    pub fn build(self, base_path: impl IntoUrl) -> anyhow::Result<ApiClient> {
        let base_path = base_path.into_url()?;

        let mut headers = self.headers.unwrap_or_default();
        if let Some(user_agent) = self.user_agent {
            headers.insert(header::USER_AGENT, user_agent.parse()?);
        } else {
            headers.insert(
                header::USER_AGENT,
                HeaderValue::from_static(concat!(
                    env!("CARGO_PKG_NAME"),
                    "/",
                    env!("CARGO_PKG_VERSION"),
                )),
            );
        }
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/json"),
        );

        let client = if let Some(client) = self.client {
            client
        } else {
            Arc::new(
                reqwest::Client::builder()
                    .default_headers(headers)
                    .build()?,
            )
        };

        Ok(ApiClient::new(client, base_path))
    }
}
