use std::{collections::HashMap, path::Path};

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

pub mod daemon;
#[async_trait]
pub trait Ipfs {
    /// This method uploads a file or directory at a given path to the IPFS swarm you are
    /// currently connected to.
    async fn add(&self, path:&Path) -> Result<HashMap<String, String>>;
    /// This method connects to the given address, adding the address to the current swarm
    async fn connect_to(&self, peer_id:&str) -> Result<()>;
    /// This method returns a list of all the addresses that are currently connected
    async fn get_connected(&self) -> Result<Vec<String>>;
    /// This method changes the value of a given property in the IPFS config
    /// 
    /// ```no_run
    /// use fission::ipfs::daemon::IpfsDaemon;
    /// use fission::ipfs::Ipfs;
    /// use futures::executor::block_on;
    /// use serde_json::Value;
    /// 
    /// let ipfs = IpfsDaemon::default();
    /// let config_value = Value::from("11GB");
    /// block_on(ipfs.set_config("Datastore.StorageMax", &config_value)).unwrap();
    /// ```
    async fn set_config(&self, property:&str, val:&Value) -> Result<()>;
    
    /// This method returns the value a property in the IPFS config
    /// 
    /// ```no_run
    /// use fission::ipfs::daemon::IpfsDaemon;
    /// use fission::ipfs::Ipfs;
    /// use futures::executor::block_on;
    /// 
    /// let ipfs = IpfsDaemon::default();
    /// let config_value = block_on(ipfs.get_config("Datastore.StorageMax")).unwrap();
    /// ```
    async fn get_config(&self, property: &str) -> Result<Value>;
}
