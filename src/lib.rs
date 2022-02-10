//! # deta-rust
//!
//! (Unofficial) Deta SDK for Rust.

use std::marker::PhantomData;

#[cfg(feature = "base")]
pub mod base;

pub struct Deta<S: Service> {
    project_id: String,
    project_key: String,
    phantom: PhantomData<S>,
}

impl<S: Service> Deta<S> {
    pub fn new(project_id: String, project_key: String) -> Self {
        Self { project_id, project_key, phantom: PhantomData }
    }
}

pub trait Service {}
