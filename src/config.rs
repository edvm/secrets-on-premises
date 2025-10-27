use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite:///data/secrets.db?mode=rwc".to_string());

        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = env::var("PORT")
            .unwrap_or_else(|_| "5150".to_string())
            .parse::<u16>()
            .map_err(|e| anyhow::anyhow!("Invalid PORT value: {}", e))?;

        Ok(Self {
            database_url,
            host,
            port,
        })
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn server_url(&self) -> String {
        format!("http://{}", self.host)
    }
}
