use std::time::Duration;

use bytes::Bytes;

use colored::Colorize;
use anyhow::Result;
use tokio::runtime::Runtime;
use reqwest::Client;
use super::options::{CmdOptions, IPFS_RETRY_ATTEMPTS};

pub struct HttpHandler{
    http_client:Client,
    tokio:Runtime
}
impl HttpHandler {
    pub fn new() -> HttpHandler{
        let client = Client::new();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        return HttpHandler{
            http_client:client,
            tokio:runtime
        };
    }
    
    pub async fn send_request(&mut self, options:&CmdOptions) -> Result<Vec<u8>>{
        let request_url =  options.get_url();
        let body = match &options.post_options {
            Some(post_options) => Bytes::copy_from_slice(post_options.body.as_slice()),
            None => Bytes::new()
        };
        println!("{}", (format!("sending get or post request to \"{}\"", request_url).green()));
        let response = self.tokio.block_on(async {
            self.http_client
                .post(request_url)
                .body(body)
                .send().await
        })?;
        let response_bytes = response.bytes().await?;
        println!("response recieved");
        anyhow::Ok(response_bytes.into())
    }
    pub async fn try_send_request(&mut self, options:&CmdOptions) -> Result<Vec<u8>>{
        let mut attempt:u16 = 1;
        'attempt_loop: loop {
            if attempt != 1 {
                std::thread::sleep(Duration::new(get_fibinaci(attempt), 0))
            }
            println!("attempting to send post request: attempt {} of {}", attempt, IPFS_RETRY_ATTEMPTS);
            let is_final_attempt = attempt >= IPFS_RETRY_ATTEMPTS;
            let response_result = self.send_request(options).await;
            let response = match is_final_attempt {
                true => response_result?,
                false => { match response_result {
                    Ok(x) => x,
                    Err(_) => {
                        attempt += 1;
                        continue 'attempt_loop
                    }
                }}
            };
            return anyhow::Ok(response);
        }
    }
    
}

fn get_fibinaci(n:u16) -> u64{
    let phi:f64 = fixed::consts::PHI.to_num();
    let numerator:f64 = phi.powi(n as i32)-((-phi).powi(n as i32));
    let denominator:f64 = f64::sqrt(5 as f64);
    return (numerator/denominator).round() as u64;
}