use crate::rspotify::prelude::*;

pub trait OAuthEndpoints: BaseClient + OAuthClient {
    fn user_endpoint(&self) {
        println!("Performing OAuth request");
        self.endpoint_request();
    }
}
