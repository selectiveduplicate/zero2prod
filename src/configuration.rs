#[derive(serde::Deserialize)]
/// Represents the database and server application settings.
pub struct Settings {
    pub database_settings: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

/// Reads configuration settings from the `configuration.yaml`` file.
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Try to read the configuration values from the config file.
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    // Try to deserialize the configuration file values into `Settings` type.
    settings.try_deserialize::<Settings>()
}
