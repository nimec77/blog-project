//! WASM frontend constants.

/// API server port.
pub const API_PORT: u16 = 8080;

/// Token storage key in localStorage.
pub const TOKEN_STORAGE_KEY: &str = "blog_token";

/// Maximum content length before truncation in post cards.
pub const MAX_CONTENT_LENGTH: usize = 200;
