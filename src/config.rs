use serde::Deserialize;
use toml;

#[derive(Deserialize, Clone)]
pub struct CdpConfig {
    pub spacename: String,
    pub get_file: bool,
    pub path_to_save: String,
    pub credential: Credential
}

#[derive(Deserialize, Clone)]
pub struct Credential {
    pub username: String,
    pub mdp: String
}