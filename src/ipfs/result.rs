use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddedData{
    bytes:i64,
    hash:String,
    name:String,
    size:String
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ledger{
    exchanged: i64,
    peer: String,
    recv: i64,
    sent: i64,
    value: i64
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpfsBlock{
    key: String,
    size: i32
}