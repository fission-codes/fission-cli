use hyper::body::{HttpBody, Bytes};
use hyper::{Uri, Client, Request, Method, Body};
use hyper::client::connect::HttpConnector;
use anyhow::{Result};
use super::options::{CmdOptions, IPFS_PORT, IPFS_ADDR};

pub struct HttpHandler{
    http_client:Client<HttpConnector>, 
}
impl HttpHandler {
    pub fn new() -> HttpHandler{
        let client = Client::new();
        return HttpHandler{
            http_client:client
        };
    }
    pub async fn send_request(&self, options:&CmdOptions) -> Result<Vec<u8>>{
        let mut arg_str:String = options.args.iter()
            .flat_map(|(prop, val)| format!("{}={}&", prop, val).chars().collect::<Vec<_>>())
            .collect();
        arg_str.pop();
        let request_url = format!("{}:{}/api/v0/{}?{}", IPFS_ADDR, IPFS_PORT, options.cmd, arg_str);
        drop(arg_str);

        let mut response = match &options.post_options {
            Some(post_options) => {
                let mut request = Request::builder()
                    .method(Method::POST)
                    .uri(request_url);
                for (key, value) in &post_options.headers {
                    request = request.header(key, value);
                }
                self.http_client.request(request.body(Body::from((&post_options.body).clone()))?).await?
            },
            None => {
                let request:Uri = request_url.parse()?;
                self.http_client.get(request).await?
            }
        };
        let response_body = response.body_mut();
        let response_bytes = response_body.data().await.unwrap_or( Ok(Bytes::new()))?.to_vec();
        anyhow::Ok(response_bytes)
    }
}