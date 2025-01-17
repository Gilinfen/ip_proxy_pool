use rand::Rng;

/// Generates a random IPv4 address.
fn generate_random_ip() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "{}.{}.{}.{}",
        rng.gen_range(1..=255),
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        rng.gen_range(0..=255)
    )
}

/// Generates a random port number.
fn generate_random_port() -> u16 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1024..=65535)
}

/// Generates a list of fake proxy IPs.
///
/// # Arguments
///
/// * `count` - The number of fake proxy IPs to generate.
///
/// # Returns
///
/// A vector of strings in the format "IP:PORT".
pub fn generate_fake_proxies(count: usize) -> Vec<String> {
    (0..count)
        .map(|_| format!("{}:{}", generate_random_ip(), generate_random_port()))
        .collect()
}
