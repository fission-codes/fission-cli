//use std::collections::{HashMap, btree_map::OccupiedError};

use async_trait::async_trait;
use std::collections::HashMap;

use anyhow::Result;

pub mod daemon;
pub mod http;
pub mod options;
pub mod config;

#[async_trait]
pub trait Ipfs {
    async fn add_file(&mut self, path:&str) -> Result<HashMap<String, String>>;
    async fn add_directory(&mut self, path:&str) -> Result<HashMap<String, String>>;
    async fn add_bootstrap(&mut self, peer_id:&str) -> Result<Vec<String>>;
    async fn connect_to(&mut self, peer_id:&str) -> Result<Vec<String>>;
    async fn disconect_from(&mut self, peer_id:&str)-> Result<Vec<String>>;
    async fn config(&mut self, options:&HashMap<&str, &str>) -> Result<()>;
}