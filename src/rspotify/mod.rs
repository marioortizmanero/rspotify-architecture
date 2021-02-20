// ------- Already implemented (simplified) -------

pub mod clients;

use std::collections::HashMap;

pub use clients::{ClientCredentialsSpotify, CodeAuthPKCESpotify, CodeAuthSpotify};

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

pub mod prelude {
    pub use super::clients::{BaseClient, BaseEndpoints, OAuthClient, OAuthEndpoints};
}
