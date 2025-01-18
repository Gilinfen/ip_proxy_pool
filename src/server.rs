use crate::make_https_request;
use std::collections::HashMap;
use std::sync::Arc;
use warp::http::Method;
use warp::{Filter, Rejection, Reply};

#[derive(serde::Deserialize)]
struct ProxyRequest {
    url: String,
    method: String,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
    proxies: Vec<String>, // 动态传递的代理列表
}

/// A basic proxy pool that operates with a given list of proxies.
struct ProxyPool {
    proxies: Vec<String>,
    index: usize,
}

impl ProxyPool {
    /// Create a new `ProxyPool` with the given proxies.
    fn new(proxies: Vec<String>) -> Self {
        Self { proxies, index: 0 }
    }

    /// Get the next proxy in the pool (round-robin).
    fn get_proxy(&mut self) -> Option<String> {
        if self.proxies.is_empty() {
            None
        } else {
            let proxy = self.proxies[self.index].clone();
            self.index = (self.index + 1) % self.proxies.len();
            Some(proxy)
        }
    }
}

/// Handles a proxy request by forwarding it through the dynamically provided proxy pool.
async fn handle_proxy_request(req: ProxyRequest) -> Result<impl warp::Reply, warp::Rejection> {
    // 创建代理池
    let mut proxy_pool = ProxyPool::new(req.proxies);

    // 获取代理地址
    let proxy_url = proxy_pool.get_proxy();

    // 发起请求，直接使用传入的 URL
    match make_https_request(
        &req.url, // 不做任何处理，直接使用传入的 URL
        &req.method,
        req.headers,
        req.body,
        proxy_url.as_deref(),
    )
    .await
    {
        Ok(response) => Ok(warp::reply::json(&response)),
        Err(err) => Err(warp::reject::custom(CustomError::ProxyRequestFailed(err))),
    }
}
use std::fmt;
use warp::http::StatusCode;
use warp::reject::Reject;

/// 通用错误类型
#[derive(Debug)]
pub enum CustomError {
    ProxyRequestFailed(String), // 代理请求失败
    InvalidRequest(String),     // 无效的请求
    UnknownError,               // 未知错误
}

impl Reject for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::ProxyRequestFailed(msg) => write!(f, "Proxy request failed: {}", msg),
            CustomError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            CustomError::UnknownError => write!(f, "An unknown error occurred"),
        }
    }
}

pub async fn handle_rejection_with_custom(
    err: Rejection,
    custom_handler: Option<
        Arc<dyn Fn(&Rejection) -> Option<(StatusCode, serde_json::Value)> + Send + Sync>,
    >,
) -> Result<impl Reply, std::convert::Infallible> {
    if let Some(handler) = custom_handler {
        if let Some((status, body)) = handler(&err) {
            let json_response = warp::reply::json(&body);
            return Ok(warp::reply::with_status(json_response, status));
        }
    }

    // 默认错误处理
    let json_response = warp::reply::json(&serde_json::json!({
        "status": "error",
        "message": "Unhandled rejection",
    }));

    Ok(warp::reply::with_status(
        json_response,
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

/// Starts a local proxy server with custom routes.
///
/// # Arguments
///
/// * `port` - The port to bind the server.
/// * `custom_routes` - A warp filter representing custom routes.
pub async fn start_proxy_server_with_custom_routes(
    port: u16,
    custom_routes: impl Filter<Extract = impl Reply, Error = Rejection> + Clone + Send + Sync + 'static,
    custom_handler: Option<
        Arc<dyn Fn(&Rejection) -> Option<(StatusCode, serde_json::Value)> + Send + Sync + 'static>,
    >,
) {
    // 默认的代理路由
    let proxy_route = warp::path("proxy")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_proxy_request)
        .recover(move |err| handle_rejection_with_custom(err, custom_handler.clone()));

    // 合并自定义路由和代理路由
    let routes = custom_routes.or(proxy_route);

    // Configure CORS with no rules
    let cors = warp::cors()
        .allow_any_origin() // 允许任意来源
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::HEAD,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(vec![
            "Content-Type",  // 必须允许的头，前端一般都会发送
            "Authorization", // 如果需要身份验证
        ]);

    // Start the server
    println!("Starting proxy server on port {}", port);
    warp::serve(routes.with(cors))
        .run(([127, 0, 0, 1], port))
        .await;
}
