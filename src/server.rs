use crate::make_https_request;
use std::collections::HashMap;
use warp::Filter;

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

/// Starts a local proxy server.
///
/// # Arguments
///
/// * `port` - The port to bind the server.
pub async fn start_proxy_server(port: u16) {
    // Define the proxy endpoint
    let proxy_route = warp::post()
        .and(warp::path("proxy"))
        .and(warp::body::json())
        .and_then(handle_proxy_request);

    // Start the server
    println!("Starting proxy server on port {}", port);
    warp::serve(proxy_route).run(([127, 0, 0, 1], port)).await;
}

/// Handles a proxy request by forwarding it through the dynamically provided proxy pool.
async fn handle_proxy_request(req: ProxyRequest) -> Result<impl warp::Reply, warp::Rejection> {
    // Create a dynamic proxy pool from the provided proxies
    let mut proxy_pool = ProxyPool::new(req.proxies);

    // Get a proxy from the pool (if available)
    let proxy_url = proxy_pool.get_proxy();

    // Make the request with or without proxy
    match make_https_request(
        &req.url,
        &req.method,
        req.headers,
        req.body,
        proxy_url.as_deref(),
    )
    .await
    {
        Ok(response) => Ok(warp::reply::json(&response)),
        Err(err) => Ok(warp::reply::json(&serde_json::json!({ "error": err }))),
    }
}
