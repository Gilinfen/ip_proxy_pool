# IP Proxy Pool

[中文](./README_CN.md) [English](./README.md)

A simple Rust library and server for handling HTTP proxy requests with support for dynamic proxy pools.

一个简单的 Rust 库和服务器，用于处理支持动态代理池的 HTTP 代理请求。

---

## Features 功能

- **Dynamic Proxy Pool 动态代理池**:
  - Proxies can be dynamically provided with each request.
  - 每次请求可以动态提供代理。
  - Supports round-robin proxy selection.
  - 支持轮询代理选择。

- **HTTP Proxy Server HTTP 代理服务器**:
  - A lightweight HTTP server to forward requests through the proxy pool.
  - 一个轻量级 HTTP 服务器，通过代理池转发请求。
  - Built on top of the `warp` framework.
  - 基于 `warp` 框架构建。

- **Asynchronous Requests 异步请求**:
  - Handles HTTP requests with support for headers, body, and multiple HTTP methods.
  - 支持带有头部、正文和多种 HTTP 方法的请求。
  - Automatically handles gzip-encoded responses.
  - 自动处理 gzip 编码的响应。

---

## Installation 安装

To use this library in your Rust project, add it as a dependency in your `Cargo.toml`:

要在你的 Rust 项目中使用该库，请在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
ip_proxy_pool = { git = "https://github.com/<your-username>/ip_proxy_pool.git" }
```

Replace `<your-username>` with your GitHub username.

将 `<your-username>` 替换为你的 GitHub 用户名。

---

## Usage 使用方法

### **As a Library 作为库**

The library provides the `start_proxy_server` function to start a local proxy server.

该库提供了 `start_proxy_server` 函数，用于启动本地代理服务器。

```rust
use ip_proxy_pool::start_proxy_server;
use tokio;

#[tokio::main]
async fn main() {
    let port = 8080;
    println!("Starting proxy server...");
    start_proxy_server(port).await;
}
```

### **Dynamic Proxy Pool Example 动态代理池示例**

You can dynamically pass a list of proxies for each request:

你可以为每个请求动态传递代理列表：

```bash
curl -X POST http://127.0.0.1:8080/proxy -H "Content-Type: application/json" -d '{
    "url": "https://httpbin.org/ip",
    "method": "GET",
    "headers": null,
    "body": null,
    "proxies": [
        "http://123.45.67.89:8080",
        "http://98.76.54.32:8081"
    ]
}'
```

---

## Endpoints 接口

### `/proxy` (POST)

Forwards an HTTP request through a dynamically provided proxy pool.

通过动态提供的代理池转发 HTTP 请求。

- **Request Body 请求体**:
  ```json
  {
      "url": "https://example.com",          // Target URL 目标地址
      "method": "GET",                      // HTTP method HTTP 方法 (e.g., GET, POST)
      "headers": { "User-Agent": "test" },  // Optional headers 可选的请求头
      "body": "Optional body content",      // Optional body (for POST/PUT requests) 可选的请求正文
      "proxies": [                          // List of proxy addresses 代理地址列表
          "http://123.45.67.89:8080",
          "http://98.76.54.32:8081"
      ]
  }
  ```

- **Response 响应**:
  ```json
  {
      "body": "Response content",
      "headers": {
          "content-type": "application/json"
      },
      "set_cookie_headers": [],
      "location_header": null
  }
  ```

---

## Running Examples 运行示例

This project includes an example server you can run locally.

该项目包含一个示例服务器，你可以在本地运行。

1. Clone the repository 克隆仓库:

   ```bash
   git clone https://github.com/<your-username>/ip_proxy_pool.git
   cd ip_proxy_pool
   ```

2. Run the example server 运行示例服务器:

   ```bash
   cargo run --example server
   ```

3. Test the server using the `/proxy` endpoint 测试服务器的 `/proxy` 接口:

   ```bash
   curl -X POST http://127.0.0.1:8080/proxy    -H "Content-Type: application/json"    -d '{
       "url": "https://httpbin.org/ip",
       "method": "GET",
       "headers": null,
       "body": null,
       "proxies": ["http://123.45.67.89:8080"]
   }'
   ```

---

## Contributing 贡献

Contributions are welcome! If you find a bug or want to suggest a feature, please open an issue or submit a pull request.

欢迎贡献代码！如果你发现了一个 bug 或者想要建议一个功能，请提交 issue 或 pull request。

---

## License 许可证

This project is licensed under the MIT License. See the `LICENSE` file for more details.

该项目使用 MIT 许可证。详情请查看 `LICENSE` 文件。
