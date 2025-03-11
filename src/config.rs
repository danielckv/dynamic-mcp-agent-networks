use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub scan: ScanConfig,
    pub connector: ConnectorConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScanConfig {
    pub networks: Vec<String>,
    pub ports: Vec<u16>,
    pub concurrency: usize,
    pub timeout_ms: u64,
    pub use_mdns: bool,
    pub use_upnp: bool,
    pub exclude_ips: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectorConfig {
    pub default_polling_interval: u64,
    pub output_format: String,
    pub include_metadata: bool,
    pub templates_dir: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file: Option<String>,
    pub console: bool,
}

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    let config_path = path.as_ref();

    // If configuration file doesn't exist, create default config
    if !config_path.exists() {
        let default_config = create_default_config();
        let toml_str = toml::to_string(&default_config)?;
        fs::write(config_path, toml_str)?;
        return Ok(default_config);
    }

    // Read and parse existing config
    let config_str = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_str)?;

    Ok(config)
}

fn create_default_config() -> Config {
    Config {
        scan: ScanConfig {
            networks: vec!["192.168.1.0/24".to_string()],
            ports: vec![
                22, 80, 443, 3306, 5432, 1521, 27017, // Common service ports
                8080, 8443, 9000, 9200, // Common web services
                514, 1468, 10514, // Syslog
                1883, 8883, 5672, // MQTT and AMQP
            ],
            concurrency: 100,
            timeout_ms: 1000,
            use_mdns: true,
            use_upnp: true,
            exclude_ips: Vec::new(),
        },
        connector: ConnectorConfig {
            default_polling_interval: 60,
            output_format: "json".to_string(),
            include_metadata: true,
            templates_dir: None,
        },
        logging: LoggingConfig {
            level: "info".to_string(),
            file: Some("mcp_scanner.log".to_string()),
            console: true,
        },
    }
}

pub fn save_config<P: AsRef<Path>>(config: &Config, path: P) -> Result<(), Box<dyn Error>> {
    let toml_str = toml::to_string(config)?;
    fs::write(path, toml_str)?;
    Ok(())
}
