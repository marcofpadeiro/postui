use reqwest::{header::HeaderMap, StatusCode};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ResponseType {
    Text,
    Application,
    #[default]
    Unknown,
}

impl ResponseType {
    pub fn from_str(str: &str) -> Self {
         match str {
            "text" => ResponseType::Text,
            "application" => ResponseType::Application,
            _ => ResponseType::Unknown,
        }
    }
}

#[derive(Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ResponseSubtype {
    Html,
    Json,
    Plain,
    Xml,
    #[default]
    Unknown,
}

impl ResponseSubtype {
    pub fn from_str(str: &str) -> Self {
        match str {
            "html" => ResponseSubtype::Html,
            "json" => ResponseSubtype::Json,
            "plain" => ResponseSubtype::Plain,
            "xml" => ResponseSubtype::Xml,
            _ => ResponseSubtype::Unknown,
        }
    }
}

pub struct ContentType {
    pub raw: String,
    pub response_type: ResponseType,
    pub response_subtype: ResponseSubtype,
}

impl ContentType {
    pub fn from_str(s: String) -> Self {
        let (response_type, response_subtype) = if let Some((t, u)) = s.split_once('/') {
            let rt = ResponseType::from_str(t);
            let rs = ResponseSubtype::from_str(u);
            (rt, rs)
        } else {
            (ResponseType::default(), ResponseSubtype::default())
        };

        ContentType {
            raw: s.clone(),
            response_type,
            response_subtype,
        }
    }
}

pub struct ResponseWrapper {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: bytes::Bytes
}

