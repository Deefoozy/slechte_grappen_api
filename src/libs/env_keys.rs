pub fn check_env_key(key: &str) -> String {
    std::env::var(key).expect(
        format!("Key \"{}\" has an issue", key.to_string())
            .as_str()
    )
}
