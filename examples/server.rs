use ip_proxy_pool::start_proxy_server;
use tokio; // 替换 `your_crate_name` 为实际的 crate 名称

#[tokio::main]
async fn main() {
    // 设置监听端口
    let port = 8080;

    // 启动动态代理服务器
    println!("Starting dynamic proxy server...");
    start_proxy_server(port).await;
}
