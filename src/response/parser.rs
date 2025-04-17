use std::{env, fs, io::Write};

use reqwest::Response;
use reqwest::header::HeaderValue;

use super::model::{ResponseSubtype, ResponseType};

const DEFAULT_RESPONSE_HTML: &str = "response.html";

pub fn parse_content_type(content_type: &HeaderValue) -> (ResponseType, ResponseSubtype) {
    let ct = content_type.to_str().unwrap_or("").to_ascii_lowercase();
    let parts: Vec<&str> = ct.split(';').next().unwrap_or("").split('/').collect();

    let (main, sub) = match parts.as_slice() {
        [main, sub] => (main.trim(), sub.trim()),
        _ => ("unknown", "unknown"),
    };

    let response_type = match main {
        "text" => ResponseType::Text,
        "application" => ResponseType::Application,
        _ => ResponseType::Unknown,
    };

    let response_subtype = match sub {
        "html" => ResponseSubtype::Html,
        "json" => ResponseSubtype::Json,
        "plain" => ResponseSubtype::Plain,
        "xml" => ResponseSubtype::Xml,
        _ => ResponseSubtype::Unknown,
    };

    (response_type, response_subtype)
}

/*
* Function that outputs html response to a file so it can be viewed in a browser
* @returns String: Response with file link
**/
pub async fn parse_html_response(response: Response) -> String {
    let file_path = env::temp_dir().join(DEFAULT_RESPONSE_HTML);
    let mut file = fs::File::create(file_path.clone()).unwrap();

    return match file.write_all(response.bytes().await.unwrap().as_ref()) {
        Ok(_) => format!(
            "file://{}",
            file_path.to_str().unwrap_or(DEFAULT_RESPONSE_HTML)
        ),
        Err(_) => String::from("Error writing to file"),
    };
}

pub async fn parse_json_response(response: Response) -> String {
    let json = response
        .text()
        .await
        .unwrap()
        .parse::<serde_json::Value>()
        .unwrap();

    return match serde_json::to_string_pretty(&json) {
        Ok(json) => json,
        Err(_) => String::from("Error parsing JSON"),
    };
}
