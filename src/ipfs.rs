//use std::collections::{HashMap, btree_map::OccupiedError};

use async_trait::async_trait;
use std::collections::HashMap;

use anyhow::Result;

pub mod daemon;
pub mod http;
pub mod options;

#[async_trait]
pub trait Ipfs {
    async fn add_file(&mut self, name:&str, file_name:&str, path:&str, contents:Vec<u8>) -> Result<String>;
    async fn add_directory(&mut self, path:&str) -> Result<String>;
    async fn add_bootstrap(&mut self, peer_id:&str) -> Result<Vec<String>>;
    async fn connect_to(&mut self, peer_id:&str) -> Result<Vec<String>>;
    async fn disconect_from(&mut self, peer_id:&str)-> Result<Vec<String>>;
    async fn config(&mut self, options:HashMap<String, String>) -> Result<HashMap<String, String>>;
}