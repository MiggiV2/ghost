use std::fmt;
use std::fmt::Formatter;

use strum_macros::EnumIter;

mod tests;
pub mod services;
pub mod config_builder;

#[derive(Debug, EnumIter)]
pub enum ServiceType {
    Synapse,
    Nextcloud,
    Forgejo,
    Portainer,
    Keycloak,
    Bitwarden,
    Wordpress,
    Gotosocial,
}

impl fmt::Display for ServiceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}