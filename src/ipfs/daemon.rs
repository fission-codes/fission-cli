use crate::ipfs::Ipfs;
use super::CidData;
use anyhow::Result;
use std::collections::HashMap;
pub struct IpfsViaDaemon {
    
}
impl IpfsViaDaemon {
    pub fn new() -> Result<IpfsViaDaemon> {
        todo!()
    }
}
impl Ipfs for IpfsViaDaemon {
    fn add_file(path:&str) -> Result<CidData> {
        todo!()
    }

    fn add_directory(path:&str) -> Result<CidData> {
        todo!()
    }

    fn add_bootstrap(peer_id:&str) -> Result<Vec<String>> {
        todo!()
    }

    fn connect_to(peer_id:&str) -> Result<Vec<String>> {
        todo!()
    }

    fn disconect_from(peer_id:&str)-> Result<Vec<String>> {
        todo!()
    }

    fn config(options:HashMap<String, String>) -> Result<HashMap<String, String>> {
        todo!()
    }
}