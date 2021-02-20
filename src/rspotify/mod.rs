// ------- Already implemented (simplified) -------

pub mod clients;
pub mod endpoints;

use std::collections::HashMap;

pub use clients::{ClientCredentialsSpotify, CodeAuthPKCESpotify, CodeAuthSpotify};

pub mod prelude {
    pub use super::clients::{BaseClient, OAuthClient};
    pub use super::endpoints::{BaseEndpoints, OAuthEndpoints};
}

#[derive(Clone, Debug)]
pub struct Credentials;
#[derive(Clone, Debug)]
pub struct OAuth;
#[derive(Clone, Debug)]
pub struct Token(String);

#[derive(Clone, Debug)]
pub struct HTTPClient;

impl HTTPClient {
    pub fn request(&self, params: HashMap<String, String>) {
        println!("Performed request with params {:?}", params);
    }
}
