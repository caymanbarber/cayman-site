use std::env;

pub struct AppConfig {
    pub port: u16,
    pub static_dir: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            static_dir: env::var("STATIC_DIR").unwrap_or_else(|_| "../frontend/dist".into()),
        }
    }
}
