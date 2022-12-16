use std::process::Command;
use std::collections::HashMap;
use std::thread::{spawn, sleep};
use std::time::{SystemTime, Duration};

use futures::executor::block_on;
use graceful::SignalGuard;
use serde_json::Value;
use tokio::runtime::Runtime;
use ipfs_api_backend_hyper::{TryFromUri, IpfsClient, IpfsApi, LoggingLevel, Logger};
use anyhow::{bail, Result};
use async_trait::async_trait;
use colored::Colorize;
use std::path::Path;

use crate::ipfs::Ipfs;
use crate::utils::config::{IPFS_ADDR, IPFS_API_PORT, IPFS_EXE, IPFS_SLEEP_LENGTH, IPFS_BOOT_TIME_OUT};

pub struct IpfsDaemon {
    client: IpfsClient,
    tokio: Runtime
}

impl IpfsDaemon {
    pub fn new() -> Result<Self> {
        let client = IpfsClient::from_host_and_port("http".parse()?, IPFS_ADDR, IPFS_API_PORT)?;

        let runtime = tokio::runtime::Runtime::new().unwrap();
        Ok(Self{
            client,
            tokio: runtime
        })   
    }
    pub async fn launch(&self) -> Result<()>{
        //launch the daemon
        let api_addr = format!("/ip4/{}/tcp/{}", IPFS_ADDR, IPFS_API_PORT);
        println!("Launhing IPFS...");
        Command::new(IPFS_EXE)
            .arg("--api")
            .arg(&api_addr)
            .arg("daemon")
            .spawn()
            .unwrap_or_else(|e| panic!("Failed to start IPFS daemon: {}\n This error may be because the Kubo binary is not on your PATH.", e));

        //Wait ipfs to ready
        println!("Waiting for IPFS to ready..");
        self.await_ready().await?;

        //Reduce log level for IPFS
        self.tokio.block_on(async {
            self.client.log_level(Logger::All, LoggingLevel::Error).await
        })?;
        
        //setup gracefull shutdown
        println!("Creating gracefull shutdown for IPFS...");
        let me = self.clone();
        spawn(move || {
            let signal_guard = SignalGuard::new();

            signal_guard.at_exit(move |sig| {
                println!("Signal {} received. Attempting to stop IPFS...", sig);
                match me.shutdown() {
                    Ok(_) => println!("{}", "IPFS has shutdown successfully.".green()),
                    Err(_) => println!("{}", "IPFS failed to shutdown succefully! You may need to stop the proccess yourself.".red())
                };
            });
        });

        println!("{}", "IPFS has launched successfully!!".green());
        Ok(())
    }
    pub fn shutdown(&self) -> Result<()>{
        self.tokio.block_on(async {
            block_on(self.client.shutdown())
        })?;
        Ok(())
    }
    async fn is_ipfs_ready(&self) -> bool {
        let res = self.tokio.block_on(async {
            self.client.config_show().await
        });
        // if res.is_ok() {
        //     println!("Config is {}", res.as_ref().unwrap());
        // }
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

impl Clone for IpfsDaemon {
    fn clone(&self) -> Self {
        return Self { client: self.client.clone(), tokio: Runtime::new().unwrap() }
    }
}

#[async_trait]
impl Ipfs for IpfsDaemon {
    async fn add(&self, path: &Path) -> Result<HashMap<String, String>> {
        let response_list = self.tokio.block_on(async {
            self.client.add_path(path).await
        })?;
        return Ok(response_list.into_iter().map(|res| {
            (res.name, res.hash)
        }).collect());
    }
    async fn connect_to(&self, peer_id: &str) -> Result<()> {
        todo!()
    }
    async fn get_config(&self, prop:&str) -> Result<Value>{
        let config = self.tokio.block_on(async {
            self.client.config_get_json(prop).await
        })?;
        return Ok(config.value);
    }
    async fn set_config(&self, prop:&str, val:&Value) -> Result<()> {
        if val.is_boolean() {
            self.tokio.block_on(async {
                self.client.config_set_bool(prop, val.as_bool().unwrap()).await
            })?;
            return Ok(());
        }
        if val.is_string() {
            self.tokio.block_on(async {
                self.client.config_set_string(prop, val.as_str().unwrap()).await
            })?;
            return Ok(());
        }
        self.tokio.block_on(async {
            self.client.config_set_json(prop, &val.to_string()).await
        })?;

        return Ok(());
    }
}