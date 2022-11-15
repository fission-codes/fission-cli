use bytes::Bytes;

use colored::Colorize;
use anyhow::Result;
use tokio::runtime::Runtime;
use reqwest::{Client, Response};
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
    pub async fn try_send_request<F, V>(&mut self, options:&CmdOptions, response_handler:F) -> Result<V>
        where F: Fn(Vec<u8>) -> Result<V>{
        let mut attempt:u8 = 1;
        'attempt_loop: loop {
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
            let handled_result = match is_final_attempt {
                true => response_handler(response),
                false => { match response_handler(response) {
                    Ok(x) => Ok(x),
                    Err(_) => {
                        attempt += 1;
                        continue 'attempt_loop
                    }
                }}
            };
            return Ok(handled_result?);
        }
    }
    
}
