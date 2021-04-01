//! Separate module for client implementations, currently the client credentials
//! flow, code authentication, and code authentication with PKCE. This also
//! declares the minimum required methods for the different kinds of clients.

// -------- To be discussed --------

pub mod base;
pub mod token;
pub mod client_creds;
pub mod code_auth;
pub mod code_auth_pkce;

pub use base::{BaseClient, OAuthClient};
pub use client_creds::ClientCredentialsSpotify;
pub use code_auth::CodeAuthSpotify;
pub use code_auth_pkce::CodeAuthPKCESpotify;
