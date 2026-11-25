//! Hand-written layer of the TorBox Rust SDK (sdk-config/overlays/rust). Not generated.
//!
//! ```no_run
//! # async fn run() -> Result<(), torbox::TorBoxError> {
//! use torbox::apis::user_api;
//! let client = torbox::Client::new("your-api-key");
//! let me = torbox::send(user_api::get_user_data(&client.config, Default::default())).await?;
//! println!("{:?}", me.data);
//! # Ok(()) }
//! ```
use std::future::Future;
use std::time::Duration;

use crate::apis::configuration::Configuration;
use crate::apis::Error as ApiError;

pub const DEFAULT_BASE_URL: &str = "https://api.torbox.app";
pub const SDK_VERSION: &str = "9.3.2026112508";

/// An answer with `success: false` or an error status. `error` is the stable code (e.g. `BAD_TOKEN`), `detail` the
/// message for people. `status` is `None` when a 200 answer said `success: false` or the request never got an answer.
#[derive(Debug, Clone)]
pub struct TorBoxError {
    pub status: Option<u16>,
    pub error: Option<String>,
    pub detail: String,
    pub data: Option<serde_json::Value>,
}

impl std::fmt::Display for TorBoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}: {}", self.status.map(|s| s.to_string()).unwrap_or_default(), self.error.clone().unwrap_or_default(), self.detail)
    }
}
impl std::error::Error for TorBoxError {}

impl<T> From<ApiError<T>> for TorBoxError {
    fn from(err: ApiError<T>) -> Self {
        match err {
            ApiError::ResponseError(rc) => {
                let body: serde_json::Value = serde_json::from_str(&rc.content).unwrap_or(serde_json::Value::Null);
                let status = Some(rc.status.as_u16());
                match body.get("detail") {
                    // FastAPI request validation (422) is {"detail": [...]}, not the envelope
                    Some(serde_json::Value::Array(items)) => TorBoxError {
                        status,
                        error: Some("VALIDATION_ERROR".into()),
                        detail: items.iter().filter_map(|i| i.get("msg").and_then(|m| m.as_str())).collect::<Vec<_>>().join("; "),
                        data: Some(body["detail"].clone()),
                    },
                    detail => TorBoxError {
                        status,
                        error: body.get("error").and_then(|e| e.as_str()).map(String::from),
                        detail: detail.and_then(|d| d.as_str()).map(String::from).unwrap_or_else(|| rc.status.to_string()),
                        data: body.get("data").filter(|d| !d.is_null()).cloned(),
                    },
                }
            }
            other => TorBoxError { status: None, error: None, detail: other.to_string(), data: None },
        }
    }
}

/// Awaits a generated API call and turns an error status, or a 200 answer with `success: false`, into
/// [`TorBoxError`]. The answer's payload is in `.data`.
pub async fn send<R, E>(call: impl Future<Output = Result<R, ApiError<E>>>) -> Result<R, TorBoxError>
where
    R: serde::Serialize,
{
    let resp = call.await.map_err(TorBoxError::from)?;
    if let Ok(v) = serde_json::to_value(&resp) {
        if v.get("success") == Some(&serde_json::Value::Bool(false)) {
            return Err(TorBoxError {
                status: None,
                error: v.get("error").and_then(|e| e.as_str()).map(String::from),
                detail: v.get("detail").and_then(|d| d.as_str()).unwrap_or_default().to_string(),
                data: v.get("data").filter(|d| !d.is_null()).cloned(),
            });
        }
    }
    Ok(resp)
}

/// The generated API configuration with the key, a timeout and retries for 429/503.
#[derive(Clone)]
pub struct Client {
    pub config: Configuration,
    api_key: String,
}

pub struct ClientBuilder {
    api_key: String,
    base_url: String,
    timeout: Duration,
    max_retries: u32,
}

impl ClientBuilder {
    pub fn base_url(mut self, url: impl Into<String>) -> Self { self.base_url = url.into(); self }
    /// Per attempt (default 30s).
    pub fn timeout(mut self, t: Duration) -> Self { self.timeout = t; self }
    /// Retries for 429/503 (default 3), honouring Retry-After.
    pub fn max_retries(mut self, n: u32) -> Self { self.max_retries = n; self }

    pub fn build(self) -> Client {
        let http = reqwest::Client::builder().timeout(self.timeout).build().expect("reqwest client");
        let client = reqwest_middleware::ClientBuilder::new(http).with(Retry { max_retries: self.max_retries }).build();
        let config = Configuration {
            base_path: self.base_url.trim_end_matches('/').to_string(),
            user_agent: Some(format!("torbox-rust/{SDK_VERSION}")),
            client,
            bearer_access_token: Some(self.api_key.clone()),
            ..Configuration::default()
        };
        Client { config, api_key: self.api_key }
    }
}

impl Client {
    /// `api_key` is the key from <https://torbox.app/settings>.
    pub fn new(api_key: impl Into<String>) -> Self { Self::builder(api_key).build() }

    pub fn builder(api_key: impl Into<String>) -> ClientBuilder {
        ClientBuilder { api_key: api_key.into(), base_url: DEFAULT_BASE_URL.into(), timeout: Duration::from_secs(30), max_retries: 3 }
    }

    /// The key; the download link endpoints take it as their `token` parameter.
    pub fn api_key(&self) -> &str { &self.api_key }
}

/// Collects every item of a list endpoint that takes offset/limit:
/// `torbox::paginate(100, |offset, limit| async move { ... returns Vec<T> ... }).await`
pub async fn paginate<T, F, Fut>(page_size: i64, mut page: F) -> Result<Vec<T>, TorBoxError>
where
    F: FnMut(i64, i64) -> Fut,
    Fut: Future<Output = Result<Vec<T>, TorBoxError>>,
{
    let mut all = Vec::new();
    let mut offset = 0;
    loop {
        let items = page(offset, page_size).await?;
        let n = items.len() as i64;
        all.extend(items);
        if n < page_size { return Ok(all); }
        offset += page_size;
    }
}

struct Retry {
    max_retries: u32,
}

#[async_trait::async_trait]
impl reqwest_middleware::Middleware for Retry {
    async fn handle(
        &self,
        req: reqwest::Request,
        extensions: &mut http::Extensions,
        next: reqwest_middleware::Next<'_>,
    ) -> reqwest_middleware::Result<reqwest::Response> {
        let mut attempt = 0u32;
        loop {
            let Some(this) = req.try_clone() else { return next.run(req, extensions).await };
            let resp = next.clone().run(this, extensions).await?;
            let status = resp.status().as_u16();
            if (status != 429 && status != 503) || attempt >= self.max_retries {
                return Ok(resp);
            }
            let wait = resp
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok())
                .map(Duration::from_secs)
                .unwrap_or_else(|| Duration::from_millis(500 * 2u64.pow(attempt)));
            tokio::time::sleep(wait).await;
            attempt += 1;
        }
    }
}
