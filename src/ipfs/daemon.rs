use crate::ipfs::Ipfs;
use super::{error::IpfsError, result::*};
use std::collections::HashMap;
pub struct IpfsViaDaemon {
    
}
impl Ipfs for IpfsViaDaemon {
    fn add(name:&str, path:&str, is_directory:bool) -> Result<AddedData, IpfsError> {
        todo!()
    }

    fn bitswap_ledger(peer_id:&str) -> Result<Ledger, IpfsError> {
        todo!()
    }

    fn bitswap_reprovide() -> Result<String, IpfsError> {
        todo!()
    }

    fn bitswap_wantlist(peer: &str) ->  Result<HashMap<String, String>, IpfsError> {
        todo!()
    }

    fn get_block(block_cid: &str) -> Result<String, IpfsError> {
        todo!()
    }

    fn put_block(data: &str, multihash_alg: &str, multihash_length: &str, is_pin:bool, allow_big_block:bool, cid_codec:Option<String>)
        -> Result<IpfsBlock, IpfsError> {
        todo!()
    }

    fn remove_block(block_cid:&str, force:bool) -> Result<String, IpfsError> {
        todo!()
    }

    fn block_stats(block_cid:&str) -> Result<IpfsBlock, IpfsError> {
        todo!()
    }

    fn get_bootstrap_peers() -> Result<Vec<String>, IpfsError> {
        todo!()
    }

    fn add_bootstrap_peer(peer:&str) -> Result<Vec<String>, IpfsError> {
        todo!()
    }

    fn add_bootstrap_defaults() -> Result<Vec<String>, IpfsError> {
        todo!()
    }

    fn remove_bootstrap_peer(peer:&str) -> Result<Vec<String>, IpfsError> {
        todo!()
    }

    fn remove_all_bootstrap_peer(peer:&str) -> Result<Vec<String>, IpfsError> {
        todo!()
    }

    fn get_object_data(path:&str, start:Option<i64>, length:Option<i64>)
        -> Result<String, IpfsError> {
        todo!()
    }

    fn cid_to_base32(cid: &str) -> Result<String, IpfsError> {
        todo!()
    }

    fn edit_config(key:&str, value:Option<&str>, is_bool:bool, is_object:bool) -> Result<serde_json::Value, IpfsError> {
        todo!()
    }

    fn apply_profile_config(profile:&str) -> Result<serde_json::Value, IpfsError> {
        todo!()
    }

    fn replace_config(path:&str) -> Result<String, IpfsError> {
        todo!()
    }

    fn get_config() -> Result<serde_json::Value, IpfsError> {
        todo!()
    }

    fn export_dag(cid: &str) -> Result<String, IpfsError> {
        todo!()
    }

    fn get_dag(cid: &str, codec: &str) -> Result<String, IpfsError> {
        todo!()
    }

    fn import_dag(path:&str, pin_roots:bool, allow_big_block:bool) -> Result<(), IpfsError> {
        todo!()
    }
}