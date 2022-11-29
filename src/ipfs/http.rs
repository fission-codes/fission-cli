use std::{
    time::Duration, 
    collections::HashMap, 
    io::Write
};

use bytes::Bytes;
use colored::Colorize;
use anyhow::Result;
use tokio::runtime::Runtime;
use hyper::{
    Client, Body, Method, Request, 
    client::HttpConnector,
    body::HttpBody
};
use crate::utils::math;

use super::options::*;


pub struct PostOptions {
    pub headers:HashMap<String, String>,
    pub body: Vec<u8>
}
pub struct HttpRequest {
    pub addr: String,
    pub args: HashMap<String, String>,
    pub is_multipart:bool,
    post_options: Vec<PostOptions>//TODO: Better name?
}
impl HttpRequest {
    pub fn new(addr: &str, args: &HashMap<&str, &str>, is_multipart:bool) -> Self{
        let owned_args:HashMap<String, String> = args.iter()
            .map(|(key, val)| (key.to_string(), val.to_string()))
            .collect();
        Self { addr:addr.to_string(), args:owned_args, post_options: vec![], is_multipart }
    }
    //TODO: Better name?
    pub fn add_body(&mut self, headers: &HashMap<&str, &str>, body: &[u8]){
        let owned_body = body.to_vec();
        let owned_headers:HashMap<String, String> = headers.iter()
            .map(|(key, val)| (key.to_string(), val.to_string()))
            .collect();
        self.post_options.push(PostOptions { headers: owned_headers, body: owned_body });
    }
    pub fn get_url(&self) -> String {
        let mut arg_str:String = self.args.iter()
            .flat_map(|(prop, val)| format!("{}={}&", prop, val).chars().collect::<Vec<_>>())
            .collect();
        arg_str.pop();
        return match arg_str.len() == 0{
            true => self.addr.to_owned(),
            false => format!("{}?{}", self.addr, arg_str)
        }
    }
    pub fn get_ipfs_addr() -> String {
        format!("http://{}:{}/api/v0", IPFS_ADDR, IPFS_API_PORT)
    }
}

pub struct HttpHandler{
    http_client:Client<HttpConnector>,
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
    
    pub async fn send_request(&mut self, options:&HttpRequest) -> Result<Vec<u8>>{
        let request_url =  options.get_url();
        println!("{}", (format!("sending get or post request to \"{}\"", request_url).blue()));
        let mut request_builder = Request::builder()
            .method(Method::POST)
            .uri(request_url);
        let request = match options.is_multipart {
            false => {
                match options.post_options.get(0) {
                    Some(options) => {
                        for (header_key, header_val) in options.headers.iter(){
                            request_builder = request_builder.header(header_key, header_val)
                        }

                        let data = options.body.clone();
                        request_builder.body(Body::from(data))?
                    },
                    None => request_builder.body(Body::from(Bytes::new()))?
                }
            }
            true => {
                let mut body_parts = vec![];
                for post_options in &options.post_options {
                    write!(body_parts, "--{}\r\n", HTTP_BOUNDARY)?;
                    for (header_prop, header_val) in &post_options.headers {
                        write!(body_parts, "{}: {}\r\n", header_prop, header_val)?;
                    }
                    let data_text = unsafe {
                        //TODO: find a way to do this safely
                        std::str::from_utf8_unchecked(post_options.body.as_slice())
                    };
                    write!(body_parts, "\r\n{}\r\n", data_text)?;
                }
                write!(body_parts, "--{}--\r\n", HTTP_BOUNDARY)?;
                match options.post_options.len() {
                    0 => request_builder.body(Body::from(Bytes::new()))?,
                    _ => {
                        request_builder
                            .header("Content-Type", &*format!("multipart/form-data; boundary=\"{}\"", HTTP_BOUNDARY))
                            .body(Body::from(body_parts))?
                    }
                }
            }
        };
        let mut response = self.tokio.block_on(async {
            self.http_client.request(request).await
        })?;
        let mut response_data: Vec<u8> = vec![];
        while let Some(chunk) = response.body_mut().data().await {
            for byte in chunk? {
                response_data.push(byte);
            }
        }
        println!("response recieved");
        anyhow::Ok(response_data)
        
    }
    pub async fn try_send_request<F>(&mut self, options:&HttpRequest, handler_option:Option<F>) -> Result<Vec<u8>>
        where F: Fn(Vec<u8>) -> Result<bool>{
        let mut attempt:u32 = 0;
        'attempt_loop: loop {
            attempt += 1;
            if attempt != 1 {
                std::thread::sleep(Duration::new(math::get_fibinaci(attempt), 0))
            }
            println!("attempting to send post request: attempt {} of {}", attempt, IPFS_RETRY_ATTEMPTS);
            let is_final_attempt = attempt >= IPFS_RETRY_ATTEMPTS as u32;
            let response_result = self.send_request(options).await;
            let response = match is_final_attempt {
                true => response_result?,
                false => { match response_result {
                    Ok(x) => x,
                    Err(_) => continue 'attempt_loop
                }}
            };
            if is_final_attempt && handler_option.as_ref().is_some(){
                (handler_option.unwrap())(response.clone())?;
            }else if handler_option.is_some(){
                let handler_result = (handler_option.as_ref().unwrap())(response.clone());
                if handler_result.is_err() {
                    continue 'attempt_loop;
                }else if !(handler_result.unwrap()) {
                    continue 'attempt_loop;
                }
            }
            return anyhow::Ok(response);
        }
    }
    
}
