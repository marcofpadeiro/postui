use std::{collections::HashMap, error::Error, fs};

use reqwest::{Method, Response};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum RequestMethod {
    GET,
    POST,
    DELETE,
    PUT,
    OPTION,
    PATCH
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Request {
    name: String,
    description: Option<String>,
    method: RequestMethod,
    url: String,
    body: Option<String>,
    headers: Option<HashMap<String, String>>,
    params: Option<HashMap<String, String>>
}

pub fn parse_file(file_path: &String) -> Result<Request, Box<dyn Error>>{
    let file: String = fs::read_to_string(file_path)?;
    let result = toml::from_str(&file)?;

    Ok(result)
}

pub async fn perform_request(request: Request) -> Result<Response, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let builder = client.request(Method::GET, request.url);

    let response = builder.send().await?;

    Ok(response)
}

