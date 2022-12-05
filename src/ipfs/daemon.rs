use super::http::HttpRequest;
use super::options::*;
use crate::ipfs::http::HttpHandler;
use crate::ipfs::Ipfs;
use crate::utils::*;
use anyhow::{bail, Result};
use async_trait::async_trait;
use colored::Colorize;
use serde_json::Value;
use std::collections::HashMap;
use std::process::{Child, Command};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

pub struct IpfsViaDaemon {
    http: HttpHandler,
    ipfs_process: Child,
    is_ipfs_ready: bool,
    connected_peers: Vec<String>,
}
impl IpfsViaDaemon {
    pub fn new() -> Result<IpfsViaDaemon> {
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
        anyhow::Ok(IpfsViaDaemon {
            ipfs_process: proccess,
            http: HttpHandler::new(),
            is_ipfs_ready: false,
            connected_peers: vec![],
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

            sleep(Duration::new(SLEEP_LENGTH as u64, 0));

            let now = SystemTime::now();
            if now.duration_since(start_time)? > Duration::new(BOOT_TIME_OUT as u64, 0) {
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
        let addr = HttpRequest::get_ipfs_addr() + "/config/show";
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
    async fn swarm_or_bootstrap_cmd(&mut self, cmd: &str, peer_id: &str) -> Result<Vec<String>> {
        let args = HashMap::from([("arg", peer_id)]);
        let addr = HttpRequest::get_ipfs_addr() + cmd;
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
}
#[async_trait]
impl Ipfs for IpfsViaDaemon {
    async fn add_file(&mut self, path: &str) -> Result<HashMap<String, String>> {
        let cmd = HttpRequest::get_ipfs_addr() + "/add";
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
        let cmd = HttpRequest::get_ipfs_addr() + "/add";
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

    async fn add_bootstrap(&mut self, peer_id: &str) -> Result<Vec<String>> {
        self.swarm_or_bootstrap_cmd("/bootstrap/add", peer_id).await
    }

    async fn connect_to(&mut self, peer_id: &str) -> Result<Vec<String>> {
        self.swarm_or_bootstrap_cmd("/swarm/connect", peer_id)
            .await?;
        self.connected_peers.push(peer_id.to_string());
        // print!("Connected Peers: ");
        // self.connected_peers.iter().for_each(|peer| print!("{}, ", peer));
        anyhow::Ok(self.connected_peers.clone())
    }

    async fn disconect_from(&mut self, peer_id: &str) -> Result<Vec<String>> {
        self.swarm_or_bootstrap_cmd("/swarm/disconnect", peer_id)
            .await?;
        self.connected_peers = self
            .connected_peers
            .iter()
            .filter_map(|peer_id_to_check| {
                let checkable = peer_id_to_check.to_string();
                match peer_id != checkable {
                    true => Some(checkable),
                    false => None,
                }
            })
            .collect();
        print!("Connected Peers: ");
        self.connected_peers
            .iter()
            .for_each(|peer| print!("{}, ", peer));
        anyhow::Ok(self.connected_peers.clone())
    }

    async fn config(&mut self, options: &HashMap<&str, &str>) -> Result<()> {
        let get_profile = HttpRequest::new(
            &(HttpRequest::get_ipfs_addr() + "/config/show"),
            &(HashMap::new()),
            false,
        );
        let profile_vec = self.send_request(&get_profile).await?;
        let profile_str = std::str::from_utf8(profile_vec.as_slice())?;
        let mut profile: Value = serde_json::from_str(profile_str)?;
        for (setting, value) in options {
            let number_value = value.to_string().parse::<f64>();
            let bool_value = value.to_string().parse::<bool>();
            let json_value = serde_json::from_str::<Value>(value);
            let value_parsed = match number_value {
                Ok(v) => Value::Number(serde_json::Number::from_f64(v).unwrap()),
                Err(_) => match bool_value {
                    Ok(v) => Value::Bool(v),
                    Err(_) => match json_value {
                        Ok(v) => v,
                        Err(_) => Value::String(value.to_string()),
                    },
                },
            };
            let location = setting.split(".").map(|s| s.to_string());
            profile = json::change_json_part(&profile, location, &value_parsed)?;
        }
        let json_str = profile.to_string();
        println!("{}", json_str.red());
        let args = HashMap::new();
        let headers = HashMap::from([
            (
                "Content-Disposition",
                " form-data; name=\"files\"; filename=\"config\"",
            ),
            ("Content-Type", "application/octet-stream"),
        ]);

        let addr = HttpRequest::get_ipfs_addr() + "/config/replace";
        let mut request = HttpRequest::new(&addr, &args, true);
        request.add_body(&headers, json_str.as_bytes());
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
impl Drop for IpfsViaDaemon {
    fn drop(&mut self) {
        self.ipfs_process.kill().unwrap();
        println!(
            "{}",
            ("Ipfs proccess closed successfully. Feel free to close app whenever. ✅"
                .bright_green())
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::file_management;
    use futures::executor::block_on;
    use std::collections::HashMap;
    // use anyhow::Result;
    // use proptest::prelude::*;
    use super::*;
    use serial_test::serial;

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

    fn connect_to_peers() -> IpfsViaDaemon {
        let mut ipfs = IpfsViaDaemon::new().unwrap();
        for peer in PEER_ADDRS {
            let result_peers = block_on(ipfs.connect_to(peer)).unwrap();
            for result_peer in result_peers {
                println!("Connected to peer! {}", result_peer);
            }
        }
        return ipfs;
    }

    #[test]
    #[serial]
    fn can_add_directory() {
        let test_dir = "./test-dir/more-tests";
        let mut ipfs = connect_to_peers();
        let hashes = block_on(ipfs.add_directory(test_dir)).unwrap();
        println!("{}", "Finnished Hashes:".green());
        for (path, hash) in &hashes {
            println!("{}: {}", path.green(), hash.blue())
        }

        let files = file_management::get_files_in(test_dir).unwrap();
        let folders = file_management::get_dirs_in(test_dir).unwrap();

        files
            .iter()
            .map(|(path, _)| path)
            .chain(folders.iter())
            .for_each(|path_to_match| {
                let fixed_path_to_match: String = path_to_match
                    .split("/")
                    .filter(|seg| seg != &"." && !seg.is_empty())
                    .map(|seg| "/".to_string() + seg)
                    .collect();
                assert!(hashes
                    .iter()
                    .any(|(path, _)| fixed_path_to_match == path.to_owned()))
            });
    }
    #[test]
    #[serial]
    fn can_add_file() {
        let test_file = "./test-dir/test.txt";
        let mut ipfs = connect_to_peers();
        let hashes = block_on(ipfs.add_file(test_file)).unwrap();
        println!("{}", "Finnished Hashes:\n".green());
        for (path, hash) in &hashes {
            println!("{}: {}", path.green(), hash.blue())
        }
        let matchable_path: String = test_file
            .split("/")
            .filter(|seg| seg != &"." && !seg.is_empty())
            .map(|seg| "/".to_string() + seg)
            .collect();
        assert!(hashes
            .iter()
            .any(|(path, _)| matchable_path == path.to_owned()));
    }
    #[test]
    #[serial]
    fn can_config() {
        let mut ipfs = IpfsViaDaemon::new().unwrap();
        block_on(ipfs.config(&HashMap::from([
            ("Datastore.StorageMax", "11GB"),
            ("Datastore.GCPeriod", "2h"),
        ])))
        .unwrap();
        block_on(ipfs.config(&HashMap::from([
            ("Datastore.StorageMax", "10GB"),
            ("Datastore.GCPeriod", "1h"),
        ])))
        .unwrap();
        assert!(true)
    }
}
