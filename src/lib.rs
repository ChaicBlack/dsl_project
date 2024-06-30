mod config;

mod conn;
pub use conn::Connection;

pub mod msg;
pub use msg::Message;

pub mod frame;
pub use frame::Frame;

mod db;
use db::Db;

mod parse;
use parse::{Parse, ParseError};

mod server;
pub use server::server;
mod client;
pub use client::client;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
