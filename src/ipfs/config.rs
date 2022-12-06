pub mod Identity;
pub mod Datastore;
pub mod Mounts;
pub mod Discovery;
pub mod Routing;
pub mod Ipns;
pub mod Gateway;
pub mod Swarm;
pub mod Pubsub;
pub mod Peering;
pub mod DNS;
pub mod Migration;
pub mod Provider;
pub mod Reprovider;
pub mod Experimental;
pub mod Plugins;
pub mod Pinning;
pub mod API;
pub mod Addresses;
pub mod AutoNAT;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize,  Deserialize)]
pub struct Config{
    #[serde(alias = "Identity")]
    identity:Identity::Identity,
    #[serde(alias = "Datastore")]
    datastore:Datastore::Datastore,
    #[serde(alias = "Mounts")]
    mounts:Mounts::Mounts,
    #[serde(alias = "Discovery")]
    discovery:Discovery::Discovery,
    #[serde(alias = "Routing")]
    routing:Routing::Routing,
    #[serde(alias = "Ipns")]
    ipns:Ipns::Ipns,
    #[serde(alias = "Gateway")]
    gateway:Gateway::Gateway,
    #[serde(alias = "Swarm")]
    swarm:Swarm::Swarm,
    #[serde(alias = "Pubsub")]
    pubsub:Pubsub::Pubsub,
    #[serde(alias = "Peering")]
    peering:Peering::Peering,
    #[serde(alias = "DNS")]
    dns:DNS::DNS,
    #[serde(alias = "Migration")]
    migration:Migration::Migration,
    #[serde(alias = "Provider")]
    provider:Provider::Provider,
    #[serde(alias = "Reprovider")]
    reprovider:Reprovider::Reprovider,
    #[serde(alias = "Experimental")]
    experimental:Experimental::Experimental,
    #[serde(alias = "Plugins")]
    plugins:Plugins::Plugins,
    #[serde(alias = "Pinning")]
    pinning:Pinning::Pinning,
    #[serde(alias = "API")]
    api:API::API,
    #[serde(alias = "Addresses")]
    addresses:Addresses::Addresses,
    #[serde(alias = "AutoNAT")]
    auto_nat:AutoNAT::AutoNAT,
    #[serde(alias = "Bootstrap")]
    bootstrap:Vec<String>
}



/*
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