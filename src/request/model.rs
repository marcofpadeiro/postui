use crate::request::url::deserialize_url;
use crate::request::url::serialize_url;
use std::collections::HashMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::url::URL;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum RequestMethod {
    #[default]
    GET,
    POST,
    DELETE,
    PUT,
    PATCH
}



#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[allow(dead_code)]
pub struct Request {
    pub name: String,
    pub description: Option<String>,
    pub method: RequestMethod,
    #[serde(serialize_with = "serialize_url", deserialize_with = "deserialize_url")]
    pub url: URL,
    pub body: Option<String>,
    pub headers: HashMap<String, String>,
    pub params: HashMap<String, String>
}


#[allow(dead_code)]
impl RequestMethod {
    pub fn as_str(&self) -> &'static str {
        match &self {
            RequestMethod::GET => "GET",
            RequestMethod::POST => "POST",
            RequestMethod::PUT => "PUT",
            RequestMethod::DELETE => "DELETE",
            RequestMethod::PATCH => "PATCH",
        }
    }

    pub fn as_reqwest_method(&self) -> Method {
        match &self {
            RequestMethod::GET => Method::GET,
            RequestMethod::POST => Method::POST,
            RequestMethod::PUT => Method::PUT,
            RequestMethod::DELETE => Method::DELETE,
            RequestMethod::PATCH => Method::PATCH,
        }
    }

    pub fn from_str(str: &str) -> RequestMethod {
        let temp = str.to_uppercase();
        match temp.as_str() {
            "GET" => RequestMethod::GET,
            "POST" => RequestMethod::POST,
            "PUT" => RequestMethod::PUT,
            "DELETE" => RequestMethod::DELETE,
            "PATCH" => RequestMethod::PATCH,
            _ => RequestMethod::GET
        }

    }
}

