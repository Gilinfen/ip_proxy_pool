use std::collections::VecDeque;
use tokio::sync::Mutex;

/// A simple proxy pool manager.
pub struct ProxyPool {
    proxies: Mutex<VecDeque<String>>,
}

impl ProxyPool {
    /// Creates a new proxy pool with a list of proxies.
    pub fn new(proxies: Vec<String>) -> Self {
        Self {
            proxies: Mutex::new(VecDeque::from(proxies)),
        }
    }

    /// Get a proxy from the pool (round-robin).
    pub async fn get_proxy(&self) -> Option<String> {
        let mut proxies = self.proxies.lock().await;
        if let Some(proxy) = proxies.pop_front() {
            proxies.push_back(proxy.clone());
            Some(proxy)
        } else {
            None
        }
    }

    /// Add a proxy to the pool.
    pub async fn add_proxy(&self, proxy: String) {
        let mut proxies = self.proxies.lock().await;
        proxies.push_back(proxy);
    }

    /// Remove a proxy from the pool.
    pub async fn remove_proxy(&self, proxy: &str) {
        let mut proxies = self.proxies.lock().await;
        proxies.retain(|p| p != proxy);
    }
}
