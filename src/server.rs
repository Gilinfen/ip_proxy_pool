use crate::make_https_request;
use std::collections::HashMap;
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

/// Custom error type for the proxy server.
#[derive(Debug)]
enum CustomError {
    ProxyRequestFailed(String),
}

impl warp::reject::Reject for CustomError {}

/// Starts a local proxy server with custom routes.
///
/// # Arguments
///
/// * `port` - The port to bind the server.
/// * `custom_routes` - A warp filter representing custom routes.
pub async fn start_proxy_server_with_custom_routes(
    port: u16,
    custom_routes: impl Filter<Extract = impl Reply, Error = Rejection> + Clone + Send + Sync + 'static,
) {
    // Define the default proxy endpoint
    let proxy_route = warp::post()
        .and(warp::path("proxy"))
        .and(warp::body::json())
        .and_then(handle_proxy_request)
        .recover(handle_rejection);

    // Combine custom routes with the proxy route
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

/// Handles rejections and returns a unified error response.
async fn handle_rejection(err: Rejection) -> Result<impl warp::Reply, std::convert::Infallible> {
    if let Some(CustomError::ProxyRequestFailed(msg)) = err.find() {
        Ok(warp::reply::json(&serde_json::json!({ "error": msg })).into_response())
    } else {
        // Return a generic error for other rejections
        Ok(
            warp::reply::json(&serde_json::json!({ "error": "Unhandled rejection" }))
                .into_response(),
        )
    }
}
