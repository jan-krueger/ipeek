use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server_address: String,
    pub geo_db_path: String,
    pub asn_db_path: String,
}

pub fn load_config<P: AsRef<std::path::Path>>(path: P) -> Result<AppConfig, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::from(path.as_ref().to_path_buf()))
        .build()?;
    settings.try_deserialize()
}
