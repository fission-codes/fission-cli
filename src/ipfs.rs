use std::collections::HashMap;

use serde_json::Value;

use self::{error::IpfsError, result::*};

pub mod daemon;
pub mod error;
pub mod result;

pub trait Ipfs {
    fn add(name:&str, path:&str, is_directory:bool) -> Result<AddedData, IpfsError>;
    fn bitswap_ledger(peer_id:&str) -> Result<Ledger, IpfsError>;
    fn bitswap_reprovide() -> Result<String, IpfsError>;
    //Note bitswap_stat is left out because it for diagonostic purposes
    fn bitswap_wantlist(peer: &str) ->  Result<HashMap<String, String>, IpfsError>;
    fn get_block(block_cid: &str) -> Result<String, IpfsError>;
    //TODO: multihash_alg should be enum
    fn put_block(data: &str, multihash_alg: &str, multihash_length: &str, is_pin:bool, allow_big_block:bool, cid_codec:Option<String>)
        -> Result<IpfsBlock, IpfsError>;
    fn remove_block(block_cid:&str, force:bool) -> Result<String, IpfsError>;
    fn block_stats(block_cid:&str) -> Result<IpfsBlock, IpfsError>;
    fn get_bootstrap_peers() -> Result<Vec<String>, IpfsError>;
    fn add_bootstrap_peer(peer:&str) -> Result<Vec<String>, IpfsError>;
    fn add_bootstrap_defaults() -> Result<Vec<String>, IpfsError>;
    fn remove_bootstrap_peer(peer:&str) -> Result<Vec<String>, IpfsError>;
    fn remove_all_bootstrap_peer(peer:&str) -> Result<Vec<String>, IpfsError>;
    fn get_object_data(path:&str, start:Option<i64>, length:Option<i64>)
        -> Result<String, IpfsError>;
    fn cid_to_base32(cid: &str) -> Result<String, IpfsError>;
    //format, bases, hashes, commands, and codes endpoints were not included as I think they are mainly for debug
    fn edit_config(key:&str, value:Option<&str>, is_bool:bool, is_object:bool) -> Result<Value, IpfsError>; 
    fn apply_profile_config(profile:&str) -> Result<Value, IpfsError>;
    fn replace_config(path:&str) -> Result<String, IpfsError>;
    fn get_config() -> Result<Value, IpfsError>;
    fn export_dag(cid: &str) -> Result<String, IpfsError>;
    //TODO: format should be enum
    fn get_dag(cid: &str, codec: &str) -> Result<String, IpfsError>;
    fn import_dag(path:&str, pin_roots:bool, allow_big_block:bool) -> Result<(), IpfsError>;
}