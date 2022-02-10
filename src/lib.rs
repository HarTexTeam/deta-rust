//! # deta-rust
//!
//! (Unofficial) Deta SDK for Rust.

#[cfg(feature = "base")]
pub mod base;
pub mod error;

pub struct Deta {
    project_id: String,
    project_key: String,
}

impl Deta {
    pub fn new(project_id: String, project_key: String) -> Self {
        Self { project_id, project_key }
    }

    #[cfg(feature = "base")]
    pub fn base(&self) -> base::Base {
        base::Base::new(self.project_id.clone(), self.project_key.clone())
    }
}
