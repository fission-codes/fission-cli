use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize,  Deserialize)]
pub struct Config{
    #[serde(alias = "API")]
    api:API,
    #[serde(alias = "Addresses")]
    addresses:Addresses,
    #[serde(alias = "AutoNAT")]
    auto_nat:AutoNAT,
    #[serde(alias = "Bootstrap")]
    bootstrap:Vec<String>
}
#[derive(Clone, Debug, Serialize,  Deserialize)]
pub struct API{
    #[serde(alias = "HTTPHeaders")]
    http_headers:HttpHeaders,
}
#[derive(Clone, Debug, Serialize,  Deserialize)]
pub struct HttpHeaders{

}
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
#[derive(Clone, Debug, Serialize,  Deserialize)]
pub struct AutoNAT{

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