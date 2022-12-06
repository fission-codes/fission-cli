use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize,  Deserialize)]
pub struct API{
    #[serde(alias = "HTTPHeaders")]
    http_headers:Value, // Note this a dynamic Value because the http header could be anything
}