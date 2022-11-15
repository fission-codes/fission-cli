use crate::ipfs::Ipfs;
use anyhow::{Result, bail};
use super::options::*;
use serde_json::Value;
use std::collections::HashMap;
use async_trait::async_trait;
use std::process::{Command, Child};
use colored::Colorize;
use crate::ipfs::http::HttpHandler;
use std::time::{Duration, SystemTime};
use std::thread::sleep;

pub struct IpfsViaDaemon {
    http:HttpHandler, 
    ipfs_process:Child,
    is_ipfs_ready:bool
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
            is_ipfs_ready: false
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
    async fn send_request<F, V>(&mut self, options:&CmdOptions, response_handler:F) -> Result<V>
    where F: Fn(Vec<u8>) -> Result<V>{
        self.await_ready().await?;
        let response = self.http.try_send_request(options, response_handler);
        anyhow::Ok(response.await?)
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
        let peer_list = self.send_request(&cmd_options, |result_data| {
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
                    anyhow::Ok(peers)
                },
                Err(_) => bail!("The was error parsing the server's response! The server's response is as follows: \n {}", result_str)
            };
            anyhow::Ok(peer_list?)
        }).await?;

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
            ("Abspath", path),
            ("Content-Disposition", &disposition),
            ("Content-Type", "application/octet-stream")
        ]);
        let cmd_options = CmdOptions::new(cmd, &args).to_post(&headers, contents.as_slice());
        let result = self.send_request(&cmd_options, |result_data| {
            anyhow::Ok(std::str::from_utf8(result_data.as_slice())?.to_string())
        }).await?;
        return anyhow::Ok(result);
    }

    async fn add_directory(&mut self, path:&str) -> Result<String> {
        todo!()
    }

    async fn add_bootstrap(&mut self, peer_id:&str) -> Result<Vec<String>> {
        self.swarm_or_bootstrap_cmd("bootstrap/add", peer_id).await
    }

    async fn connect_to(&mut self, peer_id:&str) -> Result<Vec<String>> {
        self.swarm_or_bootstrap_cmd("swarm/connect", peer_id).await
    }

    async fn disconect_from(&mut self, peer_id:&str)-> Result<Vec<String>> {
        self.swarm_or_bootstrap_cmd("swarm/disconnect", peer_id).await
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
            self.send_request(&cmd_options, |_| anyhow::Ok(())).await?;
        }
        anyhow::Ok(())
    }
}
impl Drop for IpfsViaDaemon {
    fn drop(&mut self) {
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

    use super::*;

    #[test]
    fn can_connect(){
        println!("starting test");
        let peer_id = "/dns4/production-ipfs-cluster-us-east-1-node2.runfission.com/tcp/4003/wss/p2p/12D3KooWQ2hL9NschcJ1Suqa1TybJc2ZaacqoQMBT3ziFC7Ye2BZ";
        let mut ipfs = IpfsViaDaemon::new().unwrap();
        block_on(ipfs.connect_to(peer_id)).unwrap();
        assert!(true);
    }
}