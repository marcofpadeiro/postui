use std::collections::HashMap;

use reqwest::Method;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum RequestMethod {
    GET,
    POST,
    DELETE,
    PUT,
    PATCH
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
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Request {
    pub name: String,
    pub description: Option<String>,
    pub method: RequestMethod,
    pub url: String,
    pub body: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub params: Option<HashMap<String, String>>
}
