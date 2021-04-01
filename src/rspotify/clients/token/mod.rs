pub mod cached;
pub mod refreshing;
pub mod regular;

use crate::rspotify::Token;

pub use regular::RegularTokenHandler;
pub use cached::CachedTokenHandler;
pub use refreshing::RefreshingTokenHandler;

pub trait TokenHandler {
    fn get_token(&self) -> &Token;
    fn set_token(&mut self, token: Token);
}
