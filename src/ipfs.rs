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
    /// This method adds a given address to the list bootstrap peers in the config. This
    /// fuction will not take affect until after a restart of the daemon.
    async fn add_bootstrap(&self, peer_id:&str) -> Result<()>;
    /// This method will return a list of all the addresses that are currently connected
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