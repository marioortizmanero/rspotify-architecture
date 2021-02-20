use std::collections::HashMap;

// ------- Already implemented (simplified) -------

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

// -------- To be discussed --------
// The authentication process is limited to each client, since they are mostly
// differently implemented.

#[derive(Clone, Debug)]
pub struct ClientCredentialsSpotify {
    creds: Credentials,
    tok: Option<Token>,
    http: HTTPClient,
}

#[derive(Clone, Debug)]
pub struct CodeAuthSpotify {
    creds: Credentials,
    oauth: OAuth,
    tok: Option<Token>,
    http: HTTPClient,
}

#[derive(Clone, Debug)]
pub struct CodeAuthPKCESpotify {
    creds: Credentials,
    oauth: OAuth,
    tok: Option<Token>,
    http: HTTPClient,
}

pub mod prelude {
    pub use super::{BaseClient, OAuthClient, BaseEndpoints, OAuthEndpoints};
}

pub trait BaseClient {
    fn get_http(&self) -> &HTTPClient;
    fn get_token(&self) -> &Token;
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
        params.insert("token".to_string(), self.get_token().0.clone());
        self.request(params);
    }
}


pub trait OAuthClient {
    fn get_oauth(&self) -> &OAuth;
}

pub trait BaseEndpoints: BaseClient {
    fn base_endpoint(&self) {
        println!("Performing base request");
        self.endpoint_request();
    }
}

pub trait OAuthEndpoints: BaseClient + OAuthClient {
    fn user_endpoint(&self) {
        println!("Performing OAuth request");
        self.endpoint_request();
    }
}

impl ClientCredentialsSpotify {
    pub fn new(creds: Credentials) -> Self {
        ClientCredentialsSpotify {
            creds,
            tok: None,
            http: HTTPClient {}
        }
    }

    pub fn request_token(&mut self) {
        self.tok = Some(Token("client credentials token".to_string()))
    }
}

// This could even use a macro
impl BaseClient for ClientCredentialsSpotify {
    fn get_http(&self) -> &HTTPClient {
        &self.http
    }

    fn get_token(&self) -> &Token {
        self.tok.as_ref().unwrap()
    }

    fn get_creds(&self) -> &Credentials {
        &self.creds
    }
}
impl BaseEndpoints for ClientCredentialsSpotify { }

impl CodeAuthSpotify {
    pub fn new(creds: Credentials, oauth: OAuth) -> Self {
        CodeAuthSpotify {
            creds,
            oauth,
            tok: None,
            http: HTTPClient {}
        }
    }

    pub fn prompt_for_user_token(&mut self) {
        self.tok = Some(Token("code auth token".to_string()))
    }
}

impl BaseClient for CodeAuthSpotify {
    fn get_http(&self) -> &HTTPClient {
        &self.http
    }

    fn get_token(&self) -> &Token {
        self.tok.as_ref().unwrap()
    }

    fn get_creds(&self) -> &Credentials {
        &self.creds
    }
}
impl BaseEndpoints for CodeAuthSpotify { }

// This could also be a macro (less important)
impl OAuthClient for CodeAuthSpotify {
    fn get_oauth(&self) -> &OAuth {
        &self.oauth
    }
}
impl OAuthEndpoints for CodeAuthSpotify { }

impl CodeAuthPKCESpotify {
    pub fn new(creds: Credentials, oauth: OAuth) -> Self {
        CodeAuthPKCESpotify {
            creds,
            oauth,
            tok: None,
            http: HTTPClient {}
        }
    }

    pub fn prompt_for_user_token(&mut self) {
        self.tok = Some(Token("code auth pkce token".to_string()))
    }
}

impl BaseClient for CodeAuthPKCESpotify {
    fn get_http(&self) -> &HTTPClient {
        &self.http
    }

    fn get_token(&self) -> &Token {
        self.tok.as_ref().unwrap()
    }

    fn get_creds(&self) -> &Credentials {
        &self.creds
    }
}
impl BaseEndpoints for CodeAuthPKCESpotify { }

impl OAuthClient for CodeAuthPKCESpotify {
    fn get_oauth(&self) -> &OAuth {
        &self.oauth
    }
}
impl OAuthEndpoints for CodeAuthPKCESpotify { }
