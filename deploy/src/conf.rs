use config::Config;

pub struct DeployConfig {
    pub privkey: String,
    pub relays: Vec<String>,
}

pub fn get_config() -> DeployConfig {
    let settings = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let relays = settings
        .get_array("relays")
        .unwrap()
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    let privkey = settings.get_string("privkey").unwrap();

    DeployConfig { privkey, relays }
}
