use std::{collections::HashMap, path::Path};

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

pub mod daemon;

#[cfg(test)]
pub mod tests;

#[async_trait]
pub trait Ipfs {
    async fn add(&self, path:&Path) -> Result<HashMap<String, String>>;
    async fn add_bootstrap(&self, peer_id:&str) -> Result<()>;
    async fn get_connected(&self) -> Result<Vec<String>>;
    async fn set_config(&self, property:&str, val:&Value) -> Result<()>;
    async fn get_config(&self, property:&str) -> Result<Value>;
}