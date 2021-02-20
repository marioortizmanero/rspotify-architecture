mod rspotify;

use rspotify::{
    prelude::*, ClientCredentialsSpotify, CodeAuthPKCESpotify, CodeAuthSpotify, Credentials, OAuth,
};

fn main() {
    let credentials = Credentials {};
    let oauth = OAuth {};

    let mut cc = ClientCredentialsSpotify::new(credentials.clone());
    cc.request_token();
    cc.base_endpoint();

    let mut ca = CodeAuthSpotify::new(credentials.clone(), oauth.clone());
    ca.prompt_for_user_token();
    ca.base_endpoint();
    ca.user_endpoint();

    let mut cap = CodeAuthPKCESpotify::new(credentials, oauth);
    cap.prompt_for_user_token();
    cap.base_endpoint();
    cap.user_endpoint();

    println!("Client Credentials: {:?}", cc);
    println!("Code Auth: {:?}", ca);
    println!("Code Auth PKCE: {:?}", cap);
}
