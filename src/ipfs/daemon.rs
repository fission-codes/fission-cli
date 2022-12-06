use std::{
    collections::HashMap,
    process::{Child, Command},
    thread::sleep,
    time::{Duration, SystemTime}
};

use anyhow::{bail, Result};
use async_trait::async_trait;
use colored::Colorize;
use serde_json::Value;

use crate::ipfs::{
    config::Config,
    Ipfs
};

use crate::utils::{
    config::{IPFS_BOOT_TIME_OUT, IPFS_ADDR, IPFS_API_PORT, IPFS_EXE, IPFS_SLEEP_LENGTH},
    file_management,
    http::{HttpHandler, HttpRequest},
    json
};


pub struct IpfsDaemon {
    http: HttpHandler,
    ipfs_process: Child,
    is_ipfs_ready: bool
}
impl IpfsDaemon {
    pub fn new() -> Result<IpfsDaemon> {
        println!("{}", "Starting ipfs...".green());
        let api_addr = format!("/ip4/{}/tcp/{}", IPFS_ADDR, IPFS_API_PORT);
        let proccess = Command::new(IPFS_EXE)
        .arg("--api")
        .arg(&api_addr)
        .arg("daemon")
        .spawn()
        .unwrap_or_else(|e| panic!("Failed to start IPFS daemon: {}\n This error may be because the Kubo binary is not on your PATH.", e));

        println!(
            "{}",
            ("⚠️ Warning: Ipfs Proccess Started. Please do NOT force close this app⚠️".yellow())
        );
        anyhow::Ok(IpfsDaemon {
            ipfs_process: proccess,
            http: HttpHandler::new(),
            is_ipfs_ready: false
        })
    }
    async fn send_request(&mut self, options: &HttpRequest) -> Result<Vec<u8>> {
        self.await_ready().await?;
        let result = self
            .http
            .try_send_request(
                options,
                Some(|response_data: Vec<u8>| {
                    let response_str = std::str::from_utf8(response_data.as_slice())?;
                    //TODO: do a better job of checking here
                    anyhow::Ok(!(response_str.contains("\"Type\":\"error\"")))
                }),
            )
            .await?;
        anyhow::Ok(result) //it always return Some if a handler is given
    }
    async fn await_ready(&mut self) -> Result<()> {
        if self.is_ipfs_ready == true {
            return anyhow::Ok(());
        }
        let start_time = SystemTime::now();
        loop {
            println!("{}", "checking if ipfs is ready...".green());

            if self.poll_ipfs_ready().await {
                println!("{}", "IPFS is ready!!".green());
                self.is_ipfs_ready = true;
                break;
            }

            sleep(Duration::new(IPFS_SLEEP_LENGTH as u64, 0));

            let now = SystemTime::now();
            if now.duration_since(start_time)? > Duration::new(IPFS_BOOT_TIME_OUT as u64, 0) {
                bail!(
                    "{}",
                    "Failed to start ipfs because the timeout reached!!".red()
                )
            }
        }
        anyhow::Ok(())
    }
    async fn poll_ipfs_ready(&mut self) -> bool {
        let args = HashMap::new();
        let addr = Self::get_ipfs_addr() + "/config/show";
        let cmd = HttpRequest::new(&addr, &args, false);
        let response = self.http.send_request(&cmd).await;
        return match response {
            Ok(_) => true,
            Err(e) => {
                eprint!("{}", e);
                false
            }
        };
    }
    //TODO: Better name?
    async fn swarm_cmd(&mut self, cmd: &str, peer_id: &str) -> Result<Vec<String>> {
        let args = HashMap::from([("arg", peer_id)]);
        let addr = Self::get_ipfs_addr() + cmd;
        let cmd_options = HttpRequest::new(&addr, &args, false);
        let result_data = self.send_request(&cmd_options).await?;
        let result_str = std::str::from_utf8(result_data.as_slice())?;
        let result_json:Value = match serde_json::from_str(result_str){
            Ok(x) => x,
            Err(_) => bail!("The was error parsing the server's response! The server's response is as follows: \n {}", result_str)
        };
        let peer_list = match json::value_to_vec::<String>(&result_json, "Strings"){
            Ok(peers) => {
                for peer in peers.iter() {
                    if !peer.contains("success"){
                        bail!("The following peer did not successfully connect or disconnect: {}", peer)
                    }
                }
                peers
            },
            Err(_) => bail!("The was error parsing the server's response! The server's response is as follows: \n {}", result_str)
        };

        return anyhow::Ok(peer_list);
    }
    fn response_to_hashes(response: &str) -> Result<HashMap<String, String>> {
        let mut res = response.to_string();
        let mut ret = HashMap::new();
        return loop {
            //get location to take segment to
            let json_seg_end = match res.find("}") {
                Some(seg_loc) => seg_loc,
                None => break anyhow::Ok(ret),
            };
            //get segment to convert to json and remove the segment from res
            let res_vec = res.chars().collect::<Vec<_>>();
            let json_seg = res_vec[..(json_seg_end + 1)].iter().collect::<String>();
            res = res_vec[(json_seg_end + 1)..].iter().collect();
            //convert that segment to json and value
            let json: Value = serde_json::from_str(&json_seg)?;
            let path = match json.get("Name") {
                Some(val) => "/".to_string() + &val.to_string().split("\"").collect::<String>(),
                None => bail!("Could not find Name property in ipfs's response to an add"),
            };
            let hash = match json.get("Hash") {
                Some(val) => val.to_string().split("\"").collect::<String>(),
                None => bail!("Could not find Hash property in ipfs's response to an add"),
            };
            ret.insert(path, hash);
        };
    }

    fn get_ipfs_addr() -> String {
        format!("http://{}:{}/api/v0", IPFS_ADDR, IPFS_API_PORT)
    }
}
#[async_trait]
impl Ipfs for IpfsDaemon {
    async fn add_file(&mut self, path: &str) -> Result<HashMap<String, String>> {
        let cmd = Self::get_ipfs_addr() + "/add";
        let args = HashMap::from([("quieter", "true"), ("cid-version", "1"), ("path", &path)]);
        let disposition = format!(" form-data; name=\"files\"; filename=\"{}\"", &path);
        let headers = HashMap::from([
            ("Content-Disposition", &disposition as &str),
            ("Content-Type", "application/octet-stream"),
        ]);
        let mut cmd_options = HttpRequest::new(&cmd, &args, true);
        let contents = std::fs::read(path)?;
        cmd_options.add_body(&headers, contents.as_slice());
        let result_data = self.send_request(&cmd_options).await?;
        let response = std::str::from_utf8(result_data.as_slice())?.to_string();
        let hashes = Self::response_to_hashes(&response)?;
        anyhow::Ok(hashes)
    }

    async fn add_directory(&mut self, path: &str) -> Result<HashMap<String, String>> {
        let cmd = Self::get_ipfs_addr() + "/add";
        let args = HashMap::from([("quieter", "true"), ("cid-version", "1")]);
        let mut request = HttpRequest::new(&cmd, &args, true);
        println!("{}", "Adding the following directories..".blue());

        let dirs = file_management::get_dirs_in(path)?;
        for dir in dirs {
            let disposition = format!(" form-data; name=\"files\"; filename=\"{}\"", &dir);
            let headers = HashMap::from([
                ("Content-Disposition", &disposition as &str),
                ("Content-Type", "application/x-directory"),
            ]);
            request.add_body(&headers, &[]);
            println!("{}", dir.blue());
        }

        println!("{}", "Adding the following files..".blue());
        let files = file_management::get_files_in(path)?;

        for (file_path, data) in files {
            let disposition = format!(" form-data; name=\"files\"; filename=\"{}\"", &file_path);
            let headers = HashMap::from([
                ("Content-Disposition", &disposition as &str),
                ("Content-Type", "application/octet-stream"),
            ]);
            request.add_body(&headers, data.as_slice());
            println!("{}", file_path.blue());
        }
        let response_data = self.send_request(&request).await?;
        let response = std::str::from_utf8(response_data.as_slice())?.to_string();
        let hashes = Self::response_to_hashes(&response)?;
        anyhow::Ok(hashes)
    }

    async fn connect_to(&mut self, peer_id: &str) -> Result<()> {
        self.swarm_cmd("/swarm/connect", peer_id)
            .await?;
        // print!("Connected Peers: ");
        // self.connected_peers.iter().for_each(|peer| print!("{}, ", peer));
        anyhow::Ok(())
    }
    async fn get_config(&mut self) -> Result<Config>{
        let get_profile = HttpRequest::new(
            &(Self::get_ipfs_addr() + "/config/show"),
            &(HashMap::new()),
            false,
        );
        let profile_vec = self.send_request(&get_profile).await?;
        let profile_str = std::str::from_utf8(profile_vec.as_slice())?;
        // println!("{}", profile_str.red());
        return Ok(serde_json::from_str(profile_str)?);
    }
    async fn set_config(&mut self, options: &Config) -> Result<()> {
        let args = HashMap::new();
        let addr = Self::get_ipfs_addr() + "/config/replace";
        let mut request = HttpRequest::new(&addr, &args, true);
        let headers = HashMap::from([
            (
                "Content-Disposition",
                " form-data; name=\"files\"; filename=\"config\"",
            ),
            ("Content-Type", "application/octet-stream"),
        ]);
        let body = serde_json::to_string(options)?;
        request.add_body(&headers, body.as_bytes());
        let response = self.send_request(&request).await?;
        let res_str = std::str::from_utf8(response.as_slice())?;
        if res_str.trim().is_empty() {
            return anyhow::Ok(());
        } else {
            bail!(
                "There was error changing the settings in ipfs. The failure was: {}",
                res_str.red()
            );
        }
    }
}
impl Drop for IpfsDaemon {
    fn drop(&mut self) {
        self.ipfs_process.kill().unwrap();
        println!(
            "{}",
            ("Ipfs proccess closed successfully. Feel free to close app whenever. ✅"
                .bright_green())
        )
    }
}