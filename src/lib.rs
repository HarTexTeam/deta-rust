//! # deta-rust
//!
//! (Unofficial) Deta SDK for Rust.

#[cfg(feature = "base")]
pub mod base;
pub mod error;

#[cfg(feature = "base")]
pub fn base(project_key: String) -> base::Base {
    base::Base::new(project_key)
}
