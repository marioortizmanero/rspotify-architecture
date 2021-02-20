use crate::rspotify::{prelude::*, Credentials, HTTPClient, OAuth, Token};

#[derive(Clone, Debug)]
pub struct CodeAuthPKCESpotify {
    creds: Credentials,
    oauth: OAuth,
    tok: Option<Token>,
    http: HTTPClient,
}

impl CodeAuthPKCESpotify {
    pub fn new(creds: Credentials, oauth: OAuth) -> Self {
        CodeAuthPKCESpotify {
            creds,
            oauth,
            tok: None,
            http: HTTPClient {},
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
impl BaseEndpoints for CodeAuthPKCESpotify {}

impl OAuthClient for CodeAuthPKCESpotify {
    fn get_oauth(&self) -> &OAuth {
        &self.oauth
    }
}
impl OAuthEndpoints for CodeAuthPKCESpotify {}
