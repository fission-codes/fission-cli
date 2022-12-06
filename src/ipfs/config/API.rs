use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize,  Deserialize)]
pub struct API{
    #[serde(alias = "HTTPHeaders")]
    http_headers:HttpHeaders,
}
#[derive(Clone, Debug, Serialize,  Deserialize)]
pub struct HttpHeaders{

}