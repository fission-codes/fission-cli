use std::collections::HashMap;

pub const IPFS_PORT:u16 = 4867;
pub const IPFS_ADDR:&str = "127.0.0.1";
pub const IPFS_EXE:&str = "ipfs";
pub const BOOT_TIME_OUT:u16 = 120;//In seconds
pub const SLEEP_LENGTH:u8 = 1;//In seconds
pub const READY_TEXT:&str = "Daemon is ready";

pub struct PostOptions {
    pub headers:HashMap<String, String>,
    pub body: Vec<u8>
}
pub struct CmdOptions {
    pub cmd: String,
    pub args: HashMap<String, String>,
    pub post_options: Option<PostOptions>
}
impl CmdOptions {
    pub fn new(cmd: &str, args: &HashMap<&str, &str>) -> Self{
        let owned_args:HashMap<String, String> = args.iter()
            .map(|(key, val)| (key.to_string(), val.to_string()))
            .collect();
        Self { cmd:cmd.to_string(), args:owned_args, post_options: None }
    }
    pub fn to_post(mut self, headers: &HashMap<&str, &str>, body: &[u8]) -> Self{
        let owned_body = body.to_vec();
        let owned_headers:HashMap<String, String> = headers.iter()
            .map(|(key, val)| (key.to_string(), val.to_string()))
            .collect();
        self.post_options = Some(PostOptions { headers: owned_headers, body: owned_body });
        return self;
    }
}