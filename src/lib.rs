#![warn(clippy::all)]

mod client;
pub mod model;

pub use client::{request, websocket::BitMaxWebsocket, BitMaxClient};
pub use model::Fixed9;
