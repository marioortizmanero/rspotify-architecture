//! Separate place to implement endpoints, since they can get quite long and
//! deserve a separate file or module in my opinion. Either way, this module
//! could also go inside the `clients` module, it doesn't really matter.
//!
//! This module separates the base endpoints from the OAuth ones, but they could
//! still bundled up together under the same trait in case it's complicated to
//! manage which endpoints require user authentication and which don't.

pub mod base;
pub mod oauth;

pub use base::BaseEndpoints;
pub use oauth::OAuthEndpoints;
