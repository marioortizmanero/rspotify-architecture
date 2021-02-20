use crate::rspotify::prelude::*;

pub trait BaseEndpoints: BaseClient {
    fn base_endpoint(&self) {
        println!("Performing base request");
        self.endpoint_request();
    }
}
