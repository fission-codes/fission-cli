pub mod identity;
pub mod datastore;
pub mod mounts;
pub mod discovery;
pub mod routing;
pub mod ipns;
pub mod gateway;
pub mod swarm;
pub mod pubsub;
pub mod peering;
pub mod dns;
pub mod migration;
pub mod provider;
pub mod reprovider;
pub mod experimental;
pub mod plugins;
pub mod pinning;
pub mod api;
pub mod addresses;
pub mod auto_nat;

use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize,  Deserialize)]
pub struct Config{
    #[serde(default)]
    #[serde(alias = "Identity")]
    pub identity:Value,
    #[serde(default)]
    #[serde(alias = "Datastore")]
    pub datastore:Value,
    #[serde(default)]
    #[serde(alias = "Mounts")]
    pub mounts:Value,
    #[serde(default)]
    #[serde(alias = "Discovery")]
    pub discovery:Value,
    #[serde(default)]
    #[serde(alias = "Routing")]
    pub routing:Value,
    #[serde(default)]
    #[serde(alias = "Ipns")]
    pub ipns:Value,
    #[serde(default)]
    #[serde(alias = "Gateway")]
    pub gateway:Value,
    #[serde(default)]
    #[serde(alias = "Swarm")]
    pub swarm:Value,
    #[serde(default)]
    #[serde(alias = "Pubsub")]
    pub pubsub:Value,
    #[serde(default)]
    #[serde(alias = "Peering")]
    pub peering:Value,
    #[serde(default)]
    #[serde(alias = "DNS")]
    pub dns:Value,
    #[serde(default)]
    #[serde(alias = "Migration")]
    pub migration:Value,
    #[serde(default)]
    #[serde(alias = "Provider")]
    pub provider:Value,
    #[serde(default)]
    #[serde(alias = "Reprovider")]
    pub reprovider:Value,
    #[serde(default)]
    #[serde(alias = "Experimental")]
    pub experimental:Value,
    #[serde(default)]
    #[serde(alias = "Plugins")]
    pub plugins:Value,
    #[serde(default)]
    #[serde(alias = "Pinning")]
    pub pinning:Value,
    #[serde(default)]
    #[serde(alias = "API")]
    pub api:Value,
    #[serde(alias = "Addresses")]
    pub addresses:addresses::Addresses,
    #[serde(default)]
    #[serde(alias = "AutoNAT")]
    pub auto_nat:Value,
    #[serde(default)]
    #[serde(alias = "Bootstrap")]
    pub bootstrap:Vec<String>
}



/*
Example IPFS Config

{
   "API":{
      "HTTPHeaders":{
         
      }
   },
   "Addresses":{
      "API":"/ip4/127.0.0.1/tcp/4867",
      "Announce":[
         
      ],
      "AppendAnnounce":[
         
      ],
      "Gateway":"/ip4/127.0.0.1/tcp/5742",
      "NoAnnounce":[
         
      ],
      "Swarm":[
         "/ip4/0.0.0.0/tcp/4001",
         "/ip6/::/tcp/4001",
         "/ip4/0.0.0.0/udp/4001/quic",
         "/ip6/::/udp/4001/quic"
      ]
   },
   "AutoNAT":{
      
   },
   "Bootstrap":[
      "/dnsaddr/bootstrap.libp2p.io/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN",
      "/dnsaddr/bootstrap.libp2p.io/p2p/QmQCU2EcMqAqQPR2i9bChDtGNJchTbq5TbXJJ16u19uLTa",
      "/dnsaddr/bootstrap.libp2p.io/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb",
      "/dnsaddr/bootstrap.libp2p.io/p2p/QmcZf59bWwK5XFi76CZX8cbJ4BhTzzA3gU1ZjYZcYW3dwt",
      "/ip4/104.131.131.82/tcp/4001/p2p/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ",
      "/ip4/104.131.131.82/udp/4001/quic/p2p/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ"
   ],
   "DNS":{
      "Resolvers":{
         
      }
   },
   "Datastore":{
      "BloomFilterSize":0,
      "GCPeriod":"1h",
      "HashOnRead":false,
      "Spec":{
         "mounts":[
            {
               "child":{
                  "path":"blocks",
                  "shardFunc":"/repo/flatfs/shard/v1/next-to-last/2",
                  "sync":true,
                  "type":"flatfs"
               },
               "mountpoint":"/blocks",
               "prefix":"flatfs.datastore",
               "type":"measure"
            },
            {
               "child":{
                  "compression":"none",
                  "path":"datastore",
                  "type":"levelds"
               },
               "mountpoint":"/",
               "prefix":"leveldb.datastore",
               "type":"measure"
            }
         ],
         "type":"mount"
      },
      "StorageGCWatermark":90,
      "StorageMax":"10GB"
   },
   "Discovery":{
      "MDNS":{
         "Enabled":true
      }
   },
   "Experimental":{
      "AcceleratedDHTClient":false,
      "FilestoreEnabled":false,
      "GraphsyncEnabled":false,
      "Libp2pStreamMounting":false,
      "P2pHttpProxy":false,
      "StrategicProviding":false,
      "UrlstoreEnabled":false
   },
   "Gateway":{
      "APICommands":[
         
      ],
      "HTTPHeaders":{
         "Access-Control-Allow-Headers":[
            "X-Requested-With",
            "Range",
            "User-Agent"
         ],
         "Access-Control-Allow-Methods":[
            "GET"
         ],
         "Access-Control-Allow-Origin":[
            "*"
         ]
      },
      "NoDNSLink":false,
      "NoFetch":false,
      "PathPrefixes":[
         
      ],
      "PublicGateways":null,
      "RootRedirect":"",
      "Writable":false
   },
   "Identity":{
      "PeerID":"12D3KooWGbZNXNRr7ZJkAecA445LyjZCkz1uTXne8zJCCpnDgicX"
   },
   "Internal":{
      
   },
   "Ipns":{
      "RecordLifetime":"",
      "RepublishPeriod":"",
      "ResolveCacheSize":128
   },
   "Migration":{
      "DownloadSources":[
         
      ],
      "Keep":""
   },
   "Mounts":{
      "FuseAllowOther":false,
      "IPFS":"/ipfs",
      "IPNS":"/ipns"
   },
   "Peering":{
      "Peers":null
   },
   "Pinning":{
      "RemoteServices":{
         
      }
   },
   "Plugins":{
      "Plugins":null
   },
   "Provider":{
      "Strategy":""
   },
   "Pubsub":{
      "DisableSigning":false,
      "Router":""
   },
   "Reprovider":{
      "Interval":"12h",
      "Strategy":"all"
   },
   "Routing":{
      "Methods":null,
      "Routers":null,
      "Type":"dht"
   },
   "Swarm":{
      "AddrFilters":null,
      "ConnMgr":{
         "GracePeriod":"20s",
         "HighWater":900,
         "LowWater":600,
         "Type":"basic"
      },
      "DisableBandwidthMetrics":false,
      "DisableNatPortMap":false,
      "RelayClient":{
         
      },
      "RelayService":{
         
      },
      "ResourceMgr":{
         
      },
      "Transports":{
         "Multiplexers":{
            
         },
         "Network":{
            
         },
         "Security":{
            
         }
      }
   }
}
*/