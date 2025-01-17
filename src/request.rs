use flate2::read::GzDecoder;
use reqwest::{Client, Proxy};
use serde_json::json;
use std::collections::HashMap;
use std::io::Read;

/// Makes an HTTPS request with optional headers, body, and proxy.
/// Supports handling GZIP-encoded responses.
///
/// # Arguments
///
/// * `url` - The URL to request.
/// * `method` - The HTTP method (GET, POST, etc.).
/// * `headers` - Optional headers as a `HashMap`.
/// * `body` - Optional body as a `String`.
/// * `proxy` - Optional proxy URL as a `&str` (e.g., "http://123.45.67.89:8080").
///
/// # Returns
///
/// Returns a `Result` containing a JSON object with the response details or an error message.
pub async fn make_https_request(
    url: &str,
    method: &str,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
    proxy: Option<&str>,
) -> Result<serde_json::Value, String> {
    // Create a client builder
    let mut client_builder = Client::builder();
    // Add proxy if provided
    if let Some(proxy_url) = proxy {
        println!("proxy_url {}", proxy_url);
        client_builder = client_builder
            .proxy(Proxy::http(proxy_url).map_err(|e| format!("Invalid proxy URL: {}", e))?);
    }

    // Build the client
    let client = client_builder
        .redirect(reqwest::redirect::Policy::none()) // Disable auto redirects
        .build()
        .map_err(|err| format!("Failed to build client: {}", err))?;

    let request_builder = match method.to_lowercase().as_str() {
        "get" => client.get(url),
        "post" => client.post(url),
        "put" => client.put(url),
        "head" => client.head(url),
        "delete" => client.delete(url),
        _ => return Err("Unsupported HTTP method".into()),
    };

    let request_builder = request_builder.headers(
        headers
            .map(|headers| {
                headers
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            reqwest::header::HeaderName::from_bytes(k.as_bytes()).unwrap(),
                            reqwest::header::HeaderValue::from_str(&v).unwrap(),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default(),
    );

    let request_builder = if let Some(body) = body {
        request_builder.body(body)
    } else {
        request_builder
    };

    match request_builder.send().await {
        Ok(response) => {
            let set_cookie_headers: Vec<_> = response
                .headers()
                .get_all("Set-Cookie")
                .iter()
                .map(|val| val.to_str().unwrap_or("").to_string())
                .collect();

            let headers = response
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect::<HashMap<_, _>>();

            let location_header = response
                .headers()
                .get("Location")
                .map(|val| val.to_str().unwrap_or("").to_string());

            match response.bytes().await {
                Ok(bytes) => {
                    let text = if let Some(content_encoding) = headers.get("content-encoding") {
                        if content_encoding.to_lowercase().contains("gzip") {
                            let mut decoder = GzDecoder::new(&bytes[..]);
                            let mut decoded_text = String::new();
                            decoder
                                .read_to_string(&mut decoded_text)
                                .map_err(|err| format!("Failed to decompress GZIP: {}", err))?;
                            decoded_text
                        } else {
                            String::from_utf8_lossy(&bytes).to_string()
                        }
                    } else {
                        String::from_utf8_lossy(&bytes).to_string()
                    };

                    Ok(json!({
                        "body": text,
                        "set_cookie_headers": set_cookie_headers,
                        "headers": headers,
                        "location_header": location_header,
                    }))
                }
                Err(err) => Err(format!("Failed to read response bytes: {}", err)),
            }
        }
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}
