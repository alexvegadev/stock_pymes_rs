pub fn get_secret(secret_key: &str, default_value: &str) -> String {
    std::env::var(secret_key).unwrap_or(default_value.to_string())
}