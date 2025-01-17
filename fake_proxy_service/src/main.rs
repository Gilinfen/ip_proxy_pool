mod generator;

use generator::generate_fake_proxies;
use serde::Serialize;
use warp::Filter;

/// Response structure for the fake proxies.
#[derive(Serialize)]
struct ProxyResponse {
    proxies: Vec<String>,
}

/// Starts the fake proxy server.
#[tokio::main]
async fn main() {
    // Define the `/proxies` endpoint
    let proxies_route = warp::path!("proxies" / usize)
        .and(warp::get())
        .map(|count: usize| {
            // Generate fake proxies
            let proxies = generate_fake_proxies(count);
            warp::reply::json(&ProxyResponse { proxies })
        });

    // Start the server
    let port = 8000;
    println!("Fake proxy server running on http://127.0.0.1:{}", port);
    warp::serve(proxies_route).run(([127, 0, 0, 1], port)).await;
}
