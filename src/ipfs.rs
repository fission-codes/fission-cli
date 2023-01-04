use std::{collections::HashMap, path::Path};

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

pub mod daemon;

#[cfg(test)]
pub mod tests;

#[async_trait]
pub trait Ipfs {
    /// This method upload a given file or directory at a given path to the IPFS swarm you are
    /// currently connected to.
    async fn add(&self, path:&Path) -> Result<HashMap<String, String>>;
    /// This method connect to the given address, adding the address to the current swarm
    async fn connect_to(&self, peer_id:&str) -> Result<()>;
    /// This method returns a list of all the addresses that are currently connected
    async fn get_connected(&self) -> Result<Vec<String>>;
    /// This method will change a the value of a given property in the IPFS config
    /// 
    /// Ex. `ipfs.set_config("Datastore.StorageMax", "11GB")`
    async fn set_config(&self, property:&str, val:&Value) -> Result<()>;
    /// This method will return the value a given property in the IPFS config
    /// 
    /// Ex. `ipfs.get_config("Datastore.StorageMax")`
    async fn get_config(&self, property:&str) -> Result<Value>;
}