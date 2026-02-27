/// All application strings loaded from assets/strings.toml
pub struct Strings {
    data: toml::Value,
}

impl Strings {
    pub fn load() -> Self {
        let content = include_str!("../assets/strings.toml");
        let data: toml::Value = toml::from_str(content).expect("Failed to parse strings.toml");
        Self { data }
    }

    pub fn get(&self, section: &str, key: &str) -> &str {
        self.data
            .get(section)
            .and_then(|s| s.get(key))
            .and_then(|v| v.as_str())
            .unwrap_or("")
    }
}

lazy_static::lazy_static! {
    pub static ref S: Strings = Strings::load();
}
