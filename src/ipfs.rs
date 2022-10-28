//use std::collections::{HashMap, btree_map::OccupiedError};

use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod daemon;

pub trait Ipfs {
    fn add_file(path:&str) -> Result<CidData>;
    fn add_directory(path:&str) -> Result<CidData>;
    fn add_bootstrap(peer_id:&str) -> Result<Vec<String>>;
    fn connect_to(peer_id:&str) -> Result<Vec<String>>;
    fn disconect_from(peer_id:&str)-> Result<Vec<String>>;
    fn config(options:HashMap<String, String>) -> Result<HashMap<String, String>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CidData{
    bytes:i64,
    hash:String,
    name:String,
    size:String
}