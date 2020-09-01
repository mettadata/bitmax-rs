#![warn(clippy::all)]

mod client;
pub mod model;

pub use client::{request, BitMaxClient};
pub use model::Fixed9;
