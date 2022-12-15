use std::process::{Child, Command};
use std::collections::HashMap;
use std::thread::{self, sleep};
use std::time::{SystemTime, Duration};

use futures::executor::block_on;
use tokio::runtime::Runtime;
use ipfs_api_backend_hyper::{TryFromUri, IpfsClient, IpfsApi, LoggingLevel, Logger};
use anyhow::{bail, Result};
use async_trait::async_trait;
use colored::Colorize;
use std::path::Path;

use crate::ipfs::Ipfs;
use crate::utils::config::{IPFS_ADDR, IPFS_API_PORT, IPFS_EXE, IPFS_SLEEP_LENGTH, IPFS_BOOT_TIME_OUT};


pub struct IpfsDaemon {
    proccess: Option<Child>,
    client: IpfsClient,
    tokio:Runtime
}

impl IpfsDaemon {
    pub fn new() -> Result<Self> {
        let client = IpfsClient::from_host_and_port("http".parse()?, IPFS_ADDR, IPFS_API_PORT)?;

        let runtime = tokio::runtime::Runtime::new().unwrap();
        Ok(Self{
            proccess: None,
            client,
            tokio: runtime
        })   
    }
    pub fn has_launched(&self) -> bool{
        self.proccess.is_some()
    }
    pub async fn launch(&mut self) -> Result<()>{
        let api_addr = format!("/ip4/{}/tcp/{}", IPFS_ADDR, IPFS_API_PORT);
        println!("Launhing IPFS...");
        let proccess = Command::new(IPFS_EXE)
            .arg("--api")
            .arg(&api_addr)
            .arg("daemon")
            .spawn()
            .unwrap_or_else(|e| panic!("Failed to start IPFS daemon: {}\n This error may be because the Kubo binary is not on your PATH.", e));
        self.proccess = Some(proccess);
        println!("Waiting for IPFS to ready..");
        self.await_ready().await?;
        self.tokio.block_on(async {
            self.client.log_level(Logger::All, LoggingLevel::Error).await
        })?;
        Ok(())
    }
    pub fn shutdown(&mut self) -> Result<()>{
        Ok(match self.proccess.as_mut() {
            Some(process) => {
                self.tokio.block_on(async {
                    block_on(self.client.shutdown())
                })?;
                process.wait()?;
                self.proccess = None;
            },
            None => ()
        })
    }
    async fn is_ipfs_ready(&self) -> bool {
        let res = self.tokio.block_on(async {
            self.client.config_get_json("API").await
        });
        if res.is_ok() {
            println!("Config is {}", res.as_ref().unwrap().value);
        }
        return match res {
            Ok(_) => true,
            Err(_) => false
        };
    }
    async fn await_ready(&self) -> Result<()> {
        let start_time = SystemTime::now();
        loop {
            println!("{}", "checking if ipfs is ready...".green());

            if self.is_ipfs_ready().await {
                println!("{}", "IPFS is ready!!".green());
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
}

#[async_trait]
impl Ipfs for IpfsDaemon {
    // async fn add_file(&self, path: &str) -> Result<HashMap<String, String>> {
    //     todo!()
    // }

    async fn add(&self, path: &Path) -> Result<HashMap<String, String>> {
        // let mut form = Form::default();
        // for entry_result in WalkDir::new(path) {
        //     let entry = entry_result?;
        //     form = form.add_file(entry.file_name(), entry.path())?;
        // }
        let response_list = self.tokio.block_on(async {
            self.client.add_path(path).await
        })?;
        //let response_list = self.client.add_path(path).await?;
        return Ok(response_list.into_iter().map(|res| {
            (res.name, res.hash)
        }).collect());
    }

    async fn connect_to(&self, peer_id: &str) -> Result<()> {
        todo!()
    }

    // async fn get_config(&self) -> Result<Config>{
    //     todo!()
    // }

    // async fn set_config(&self, options: &Config) -> Result<()> {
    //     todo!()
    // }
}