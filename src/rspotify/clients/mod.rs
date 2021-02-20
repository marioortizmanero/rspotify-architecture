//! Separate module for client implementations, currently the client credentials
//! flow, code authentication, and code authentication with PKCE. This also
//! declares the minimum required methods for the different kinds of clients.

// -------- To be discussed --------

pub mod client_creds;
pub mod code_auth;
pub mod code_auth_pkce;

use crate::rspotify::{Credentials, HTTPClient, OAuth, Token};
use std::collections::HashMap;

pub use client_creds::ClientCredentialsSpotify;
pub use code_auth::CodeAuthSpotify;
pub use code_auth_pkce::CodeAuthPKCESpotify;

// The authentication process is limited to each client, since they are mostly
// differently implemented.

pub trait BaseClient {
    fn get_http(&self) -> &HTTPClient;
    fn get_token(&self) -> Option<&Token>;
    fn get_creds(&self) -> &Credentials;

    // Existing
    fn request(&self, mut params: HashMap<String, String>) {
        let http = self.get_http();
        params.insert("url".to_string(), "...".to_string());
        http.request(params);
    }

    // Existing
    fn endpoint_request(&self) {
        let mut params = HashMap::new();
        params.insert("token".to_string(), self.get_token().unwrap().0.clone());
        self.request(params);
    }
}

pub trait OAuthClient {
    fn get_oauth(&self) -> &OAuth;
}
