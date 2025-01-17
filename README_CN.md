# IP Proxy Pool
[中文](./README_CN.md) [English](./README.md)

一个简单的 Rust 库和服务器，用于处理支持动态代理池的 HTTP 代理请求。

---

## 功能

- **动态代理池**:
  - 每次请求可以动态提供代理。
  - 支持轮询代理选择。

- **HTTP 代理服务器**:
  - 一个轻量级 HTTP 服务器，通过代理池转发请求。
  - 基于 `warp` 框架构建。

- **异步请求**:
  - 支持带有头部、正文和多种 HTTP 方法的请求。
  - 自动处理 gzip 编码的响应。

---

## 安装

要在你的 Rust 项目中使用该库，请在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
ip_proxy_pool = { git = "https://github.com/<your-username>/ip_proxy_pool.git" }
```

将 `<your-username>` 替换为你的 GitHub 用户名。

---

## 使用方法

### **作为库**

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

### **动态代理池示例**

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

## 接口

### `/proxy` (POST)

通过动态提供的代理池转发 HTTP 请求。

- **请求体**:
  ```json
  {
      "url": "https://example.com",          // 目标地址
      "method": "GET",                      // HTTP 方法 (如 GET, POST)
      "headers": { "User-Agent": "test" },  // 可选的请求头
      "body": "Optional body content",      // 可选的请求正文
      "proxies": [                          // 代理地址列表
          "http://123.45.67.89:8080",
          "http://98.76.54.32:8081"
      ]
  }
  ```

- **响应**:
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

## 运行示例

该项目包含一个示例服务器，你可以在本地运行。

1. 克隆仓库:

   ```bash
   git clone https://github.com/<your-username>/ip_proxy_pool.git
   cd ip_proxy_pool
   ```

2. 运行示例服务器:

   ```bash
   cargo run --example server
   ```

3. 测试服务器的 `/proxy` 接口:

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

## 贡献

欢迎贡献代码！如果你发现了一个 bug 或者想要建议一个功能，请提交 issue 或 pull request。

---

## 许可证

该项目使用 MIT 许可证。详情请查看 `LICENSE` 文件。
