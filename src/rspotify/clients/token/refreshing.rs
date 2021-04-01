use super::TokenHandler;
use crate::rspotify::{Token, HTTPClient};

/// Has access to the HTTP client in order to refresh it automatically.
pub struct RefreshingTokenHandler<'a> {
    token: Option<Token>,
    http: &'a HTTPClient
}

impl<'a> TokenHandler for RefreshingTokenHandler<'_> {
    fn get_token(&self) -> &Token {
        self.token.as_ref().unwrap()
    }

    fn set_token(&mut self, token: Token) {
        self.token = Some(token.to_owned());
    }
}
