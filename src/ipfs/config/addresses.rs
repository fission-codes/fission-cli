use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize,  Deserialize)]
pub struct Addresses{
    #[serde(default)]
    #[serde(alias = "API")]
    api:String,
    #[serde(default)]
    #[serde(alias = "Announce")]
    announce:Vec<String>,
    #[serde(default)]
    #[serde(alias = "AppendAnnounce")]
    append_announce:Option<Vec<String>>,
    #[serde(default)]
    #[serde(alias = "Gateway")]
    gateway:String,
    #[serde(default)]
    #[serde(alias = "NoAnounce")]
    no_anounce:Option<Vec<String>>,
    #[serde(default)]
    #[serde(alias = "Swarm")]
    swarm:Vec<String>
}