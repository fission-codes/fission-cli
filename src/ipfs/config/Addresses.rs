use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize,  Deserialize)]
pub struct Addresses{
    #[serde(alias = "API")]
    api:String,
    #[serde(alias = "Announce")]
    announce:Vec<String>,
    #[serde(alias = "AppendAnnounce")]
    append_announce:Vec<String>,
    #[serde(alias = "Gateway")]
    gateway:String,
    #[serde(alias = "NoAnounce")]
    no_anounce:Vec<String>,
    #[serde(alias = "Swarm")]
    swarm:Vec<String>
}