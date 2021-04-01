use super::TokenHandler;
use crate::rspotify::Token;

pub struct RegularTokenHandler {
    token: Option<Token>
}

impl TokenHandler for RegularTokenHandler {
    fn get_token(&self) -> &Token {
        self.token.as_ref().unwrap()
    }

    fn set_token(&mut self, token: Token) {
        self.token = Some(token.to_owned());
    }
}
