use crate::ipfs::Ipfs;
use anyhow::{Result, bail};
use super::options::*;
use serde_json::Value;
use futures::{join, Future};
use std::io::Read;
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
    is_ipfs_ready:bool,
    last_ipfs_output:Vec<u8>
}
impl IpfsViaDaemon {
    pub fn new() -> Result<IpfsViaDaemon> {
        IpfsViaDaemon::configure()?;
        let proccess = Command::new(IPFS_EXE)
        .arg("daemon")
        .spawn()?;
        println!("{}", ("⚠️ Warning: Ipfs Proccess Started. Please do NOT force close this app⚠️".yellow()));
        anyhow::Ok(IpfsViaDaemon{
            ipfs_process: proccess, 
            http: HttpHandler::new(), 
            is_ipfs_ready: false,
            last_ipfs_output: vec![]
        })
    }
    fn configure() -> Result<()>{
        //This sets the API's address
        let address = format!("/ip4/{}/tcp/{}", IPFS_ADDR, IPFS_PORT);
        let is_addr_set = Command::new(IPFS_EXE)
            .arg("config")
            .arg("Addresses.API")
            .arg(address)
            .spawn()?
            .wait()?
            .success();
        if !is_addr_set {
            bail!("Attempted to set the api address, but failed!");
        }
        anyhow::Ok(())
    }
    async fn send_request(&mut self, options:&CmdOptions) -> Result<Vec<u8>>{
        self.await_ready()?;
        let response = self.http.send_request(options);
        anyhow::Ok(response.await?)
    }
    fn await_ready(&mut self) -> Result<()>{
        if self.is_ipfs_ready == true { return anyhow::Ok(());}
        let start_time = SystemTime::now();
        loop {
            match &mut self.ipfs_process.stdout {
                Some(out) => {
                    let mut buffer:Vec<u8> = vec![];
                    out.read_to_end(&mut buffer)?;
                    self.last_ipfs_output.append(&mut buffer);
                    drop(buffer);

                    let text_so_far = match std::str::from_utf8(self.last_ipfs_output.as_slice()){
                        Ok(text) => text,
                        Err(_) => ""
                    };
                    if text_so_far.contains(READY_TEXT){
                        self.is_ipfs_ready = true;
                        break;
                    }else if text_so_far.contains("\n") {
                        self.last_ipfs_output.clear();
                    }
                },
                None => ()
            }

            sleep(Duration::new(SLEEP_LENGTH as u64, 0));

            let now = SystemTime::now();
            if now.duration_since(start_time)? > Duration::new(BOOT_TIME_OUT as u64, 0) {
                bail!("{}","Failed to start ipfs because the timeout reached!!".red())
            }
        }
        anyhow::Ok(())
    }
    async fn swarm_or_bootstrap_cmd(&mut self, cmd:&str, peer_id:&str) -> Result<Vec<String>>{
        let args = HashMap::from([
            ("arg", peer_id)
        ]);
        let cmd_options = CmdOptions::new(cmd, &args);
        let result_data = self.send_request(&cmd_options).await?;
        let result_json:Value = serde_json::from_str(std::str::from_utf8(result_data.as_slice())?)?;
        let peer_list:Vec<String> = value_to_vec(&result_json, "peer")?;
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
        let result_data = self.send_request(&cmd_options).await?;
        return anyhow::Ok(std::str::from_utf8(result_data.as_slice())?.to_string());
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
            self.send_request(&cmd_options).await?;
        }
        anyhow::Ok(())
    }
}
impl Drop for IpfsViaDaemon {
    fn drop(&mut self) {
        self.ipfs_process.kill().unwrap();
        println!("{}", ("Ipfs proccess closed. Feel free to close app whenever. ✅".bright_green()))
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