use crate::ipfs::Ipfs;
use anyhow::{Result, bail};
use futures::executor::block_on;
use reqwest::header;
use super::options::*;
use serde_json::Value;
use std::collections::HashMap;
use async_trait::async_trait;
use std::process::{Command, Child};
use colored::Colorize;
use crate::ipfs::http::HttpHandler;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use walkdir::WalkDir;

pub struct IpfsViaDaemon {
    http:HttpHandler, 
    ipfs_process:Child,
    is_ipfs_ready:bool,
    connected_peers:Vec<String>
}
impl IpfsViaDaemon {
    pub fn new() -> Result<IpfsViaDaemon> {
        println!("{}", "Configuring ipfs...".green());
        IpfsViaDaemon::configure()?;
        println!("{}", "Starting ipfs...".green());
        let proccess = Command::new(IPFS_EXE)
        .arg("daemon")
        .spawn()?;
        println!("{}", ("⚠️ Warning: Ipfs Proccess Started. Please do NOT force close this app⚠️".yellow()));
        anyhow::Ok(IpfsViaDaemon{
            ipfs_process: proccess, 
            http: HttpHandler::new(), 
            is_ipfs_ready: false,
            connected_peers: vec![]
        })
    }
    fn configure() -> Result<()>{
        //This sets the API's address
        //let api_addr = format!("/ip4/{}/tcp/{}", IPFS_ADDR, IPFS_API_PORT);
        let api_addr = format!("/ip4/{}/tcp/{}", IPFS_ADDR, IPFS_API_PORT);
        let http_addr = format!("/ip4/{}/tcp/{}", IPFS_ADDR, IPFS_HTTP_PORT);
        //let http_addr = format!("/ip4/{}/tcp/{}", IPFS_ADDR, IPFS_HTTP_PORT);
        IpfsViaDaemon::config_option("Addresses.API", &api_addr)?;
        IpfsViaDaemon::config_option("Addresses.Gateway", &http_addr)?;
        anyhow::Ok(())
    }
    fn config_option(option:&str, value:&str)-> Result<()>{
        let cmd_str = format!("ipfs {} Addresses.API {}", option, value);
        println!("running cmd \"{}\"", cmd_str.blue());
        let is_addr_set = Command::new(IPFS_EXE)
            .arg("config")
            .arg(option)
            .arg(value)
            .spawn()?
            .wait()?
            .success();
        if !is_addr_set {
            bail!("Attempted to set the api address, but failed!");
        }
        anyhow::Ok(())
    }
    async fn send_request(&mut self, options:&CmdOptions) -> Result<Vec<u8>>{
        self.await_ready().await?;
        let result = self.http.try_send_request(options, Some(|response_data:Vec<u8>| {
            let response_str = std::str::from_utf8(response_data.as_slice())?;
            //TODO: do a better job of checking here
            anyhow::Ok(!(response_str.contains("\"Type\":\"error\"")))
        })).await?;
        anyhow::Ok(result)//it always return Some if a handler is given
    }
    async fn await_ready(&mut self) -> Result<()>{
        if self.is_ipfs_ready == true { return anyhow::Ok(());}
        let start_time = SystemTime::now();
        loop {
            println!("{}", "checking if ipfs is ready...".green());
            
            if self.poll_ipfs_ready().await {
                println!("{}", "IPFS is ready!!".green());
                self.is_ipfs_ready = true;
                break;
            }

            sleep(Duration::new(SLEEP_LENGTH as u64, 0));

            let now = SystemTime::now();
            if now.duration_since(start_time)? > Duration::new(BOOT_TIME_OUT as u64, 0) {
                bail!("{}","Failed to start ipfs because the timeout reached!!".red())
            }
        }
        anyhow::Ok(())
    }
    async fn poll_ipfs_ready(&mut self) -> bool{
        let args = HashMap::new();
        let cmd = CmdOptions::new("config/show", &args);
        let response = self.http.send_request(&cmd).await;
        return match response {
            Ok(_) => true,
            Err(e) => {
                eprint!("{}", e); 
                false
            }
        };
    }
    async fn swarm_or_bootstrap_cmd(&mut self, cmd:&str, peer_id:&str) -> Result<Vec<String>>{
        let args = HashMap::from([
            ("arg", peer_id)
        ]);
        let cmd_options = CmdOptions::new(cmd, &args);
        let result_data = self.send_request(&cmd_options).await?;
        let result_str =std::str::from_utf8(result_data.as_slice())?;
        let result_json:Value = match serde_json::from_str(result_str){
            Ok(x) => x,
            Err(_) => bail!("The was error parsing the server's response! The server's response is as follows: \n {}", result_str)
        };
        let peer_list = match value_to_vec::<String>(&result_json, "Strings"){
            Ok(peers) => {
                for peer in peers.iter() {
                    if !peer.contains("success"){
                        bail!("The following peer did not successfully connect: {}", peer)
                    }
                }
                peers
            },
            Err(_) => bail!("The was error parsing the server's response! The server's response is as follows: \n {}", result_str)
        };

        return anyhow::Ok(peer_list);
    }
}
#[async_trait]
impl Ipfs for IpfsViaDaemon {
    async fn add_file(&mut self, name:&str, file_name:&str, path:&str, contents:Vec<u8>) -> Result<String> {
        let cmd = "add";
        let args = HashMap::from([
            ("quieter", "true"),
            ("cid-version", "1")
        ]);
        let disposition = format!("form-data; name=\"{}\"; filename=\"{}\"", name, file_name);
        let headers = HashMap::from([
            ("Content-Type", "application/octet-stream"),
            ("Content-Disposition", &disposition)
        ]);
        let cmd_options = CmdOptions::new(cmd, &args).to_post(&headers, contents.as_slice());
        let result_data = self.send_request(&cmd_options).await?;
        let result = std::str::from_utf8(result_data.as_slice())?.to_string();
        return anyhow::Ok(result);
    }

    async fn add_directory(&mut self, path:&str) -> Result<String> {
        for entry in WalkDir::new(path) {
            print!("{}", std::fs::read_to_string(entry?.path())?)
        }
        anyhow::Ok("Done".to_string())
    }

    async fn add_bootstrap(&mut self, peer_id:&str) -> Result<Vec<String>> {
        self.swarm_or_bootstrap_cmd("bootstrap/add", peer_id).await
    }

    async fn connect_to(&mut self, peer_id:&str) -> Result<Vec<String>> {
        let response = self.swarm_or_bootstrap_cmd("swarm/connect", peer_id).await?;
        self.connected_peers.push(peer_id.to_string());
        print!("Connected Peers: ");
        self.connected_peers.iter().for_each(|peer| print!("{}, ", peer));
        anyhow::Ok(response)
    }

    async fn disconect_from(&mut self, peer_id:&str)-> Result<Vec<String>> {
        let response = self.swarm_or_bootstrap_cmd("swarm/disconnect", peer_id).await?;
        self.connected_peers.iter_mut()
            .filter(|peer_id_to_check| (peer_id_to_check.to_string() != peer_id.to_string()));
        print!("Connected Peers: ");
        self.connected_peers.iter().for_each(|peer| print!("{}, ", peer));
        anyhow::Ok(response)
    }

    async fn config(&mut self, options:HashMap<String, String>) -> Result<()> {
        for (setting, value) in options {
            let cleaned_value = value.to_lowercase();
            let is_bool = match cleaned_value.trim() == "true" || cleaned_value.trim() == "false" {
                true => "true",
                false => "false"
            };
            let is_json = match serde_json::from_str::<Value>(&value).is_ok() {
                true => "true",
                false => "false"
            };
            let args = HashMap::from([
                ("arg".to_string(), setting.to_string()),
                ("arg".to_string(), value.to_string()),
                ("bool".to_string(), is_bool.to_string()),
                ("json".to_string(), is_json.to_string())
            ]);
            let cmd_options = CmdOptions{
                cmd:"config".to_string(),
                post_options:None,
                args
            };
            self.send_request(&cmd_options).await?;
        }
        anyhow::Ok(())
    }
}
impl Drop for IpfsViaDaemon {
    fn drop(&mut self) {
        for peer in self.connected_peers.clone(){
            block_on(self.disconect_from(&peer));
        }
        self.ipfs_process.kill().unwrap();
        println!("{}", ("Ipfs proccess closed successfully. Feel free to close app whenever. ✅".bright_green()))
    }
}
//TODO: This needs to be move to utils
fn value_to_vec<A>(json:&Value, index:&str) -> Result<Vec<A>> 
    where A:Clone + for<'de> serde::Deserialize<'de>{
    anyhow::Ok( match json.get(index) {
        Some(list_json) => { 
            match list_json.as_array() {
                Some(list) => {
                    list.iter().map(|item_json| {
                        let item:A = match serde_json::from_value(item_json.clone()){
                            Ok(x) => x,
                            Err(_) => panic!("Improperly formatted Json: cannot format type")
                        };
                        item
                    }).collect()
                },
                None => panic!("Improperly formatted Json: Not an Array")
            }
        },
        None => bail!("Improperly formatted Json: Cant locate index {}", index)
    })   
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use anyhow::Result;
    use proptest::prelude::*;
    use super::*;

    //TODO: Setup pizza 
    const PEER_ADDRS:&'static  [&'static str] = &[
        "/dns4/production-ipfs-cluster-us-east-1-node2.runfission.com/tcp/4003/wss/p2p/12D3KooWQ2hL9NschcJ1Suqa1TybJc2ZaacqoQMBT3ziFC7Ye2BZ",
        "/dns4/production-ipfs-cluster-eu-north-1-node1.runfission.com/tcp/4003/wss/p2p/12D3KooWRwbRrSN2cPAKz4yt1vxBFdh53CpgWjSFK5hZPkzHHz5h",
        "/dns4/production-ipfs-cluster-eu-north-1-node1.runfission.com/tcp/4001/p2p/12D3KooWRwbRrSN2cPAKz4yt1vxBFdh53CpgWjSFK5hZPkzHHz5h",
        "/dns4/production-ipfs-cluster-mega-us-east-1-node0.runfission.com/tcp/4001/p2p/12D3KooWJQHUo1snJrv5NWVesFspjwhkNkaMu5M9cMdF1oF7ucTz",
        "/ip4/54.235.17.70/tcp/4001/p2p/12D3KooWJQHUo1snJrv5NWVesFspjwhkNkaMu5M9cMdF1oF7ucTz",
        "/ip4/54.235.17.70/udp/4001/quic/p2p/12D3KooWJQHUo1snJrv5NWVesFspjwhkNkaMu5M9cMdF1oF7ucTz",
        "/dns4/production-ipfs-cluster-mega-us-east-1-node0.runfission.com/tcp/4003/wss/p2p/12D3KooWJQHUo1snJrv5NWVesFspjwhkNkaMu5M9cMdF1oF7ucTz",
        "/dns4/production-ipfs-cluster-us-east-1-node2.runfission.com/tcp/4001/p2p/12D3KooWQ2hL9NschcJ1Suqa1TybJc2ZaacqoQMBT3ziFC7Ye2BZ",
        "/dns4/production-ipfs-cluster-eu-north-1-node0.runfission.com/tcp/4001/p2p/12D3KooWDTUTdVJfW7Rwb6kKhceEwevTatPXnavPwkfZp2A6r1Fn",
        "/dns4/production-ipfs-cluster-us-east-1-node1.runfission.com/tcp/4001/p2p/12D3KooWNntMEXRUa2dNgkQsVgzao6zGSYxm1oAs83YtRy6uBuxv",
        "/dns4/production-ipfs-cluster-eu-north-1-node0.runfission.com/tcp/4003/wss/p2p/12D3KooWDTUTdVJfW7Rwb6kKhceEwevTatPXnavPwkfZp2A6r1Fn",
        "/ip4/54.235.17.70/tcp/4003/wss/p2p/12D3KooWJQHUo1snJrv5NWVesFspjwhkNkaMu5M9cMdF1oF7ucTz",
        "/dns4/production-ipfs-cluster-us-east-1-node1.runfission.com/tcp/4003/wss/p2p/12D3KooWNntMEXRUa2dNgkQsVgzao6zGSYxm1oAs83YtRy6uBuxv",
    ];

    fn connect_to_peers() -> IpfsViaDaemon{
        let mut ipfs = IpfsViaDaemon::new().unwrap();
        for peer in PEER_ADDRS {
            block_on(ipfs.connect_to(peer)).unwrap();
        }
        ipfs
        
    }

    #[test]
    fn can_connect(){
        connect_to_peers();
        assert!(true);
    }
    // proptest! {
    //     #[test]
    //     #[serial]
    //     fn can_add_file(s: &str){
    //         let mut ipfs = connect_to_peers();
    //         ipf.add_file()
    //         assert!(true);
    //     }
    // }
    #[test]
    fn can_add_directory(){
        let testdir = "./test-dir";
        let mut ipfs = connect_to_peers();
        block_on(ipfs.add_directory(testdir)).unwrap();
        assert!(true);
    }
}