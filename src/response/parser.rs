use std::{env, fs};

use reqwest::{header::CONTENT_TYPE, Response};
use serde_json::Value;

use super::model::{ContentType, ResponseSubtype, ResponseType, ResponseWrapper};

const DEFAULT_RESPONSE_HTML: &str = "response.html";

impl ResponseWrapper {
    pub async fn from_response(response: Response) -> Self {
        Self {
            status: response.status(),
            headers: response.headers().clone(),
            body: response.bytes().await.unwrap_or_else(|_| bytes::Bytes::new()),
        }
    }

    pub async fn parse_response(&self) -> String {
        let content_type = self.get_content_type();
        match (content_type.response_type, content_type.response_subtype) {
            (ResponseType::Text, ResponseSubtype::Html) => self.parse_html_response().await,
            (ResponseType::Application, ResponseSubtype::Json) => self.parse_json_response().await,
            _ => String::from_utf8(self.body.to_vec()).unwrap_or_else(|_| "Error decoding body".into()),
        }
    }

    /*
    * Function that outputs html response to a file so it can be viewed in a browser
    * @returns String: Response with file link
    **/
    async fn parse_html_response(&self) -> String {
        let file_path = env::temp_dir().join(DEFAULT_RESPONSE_HTML);
        match fs::write(&file_path, &self.body) {
            Ok(_) => format!("file://{}", file_path.to_str().unwrap_or(DEFAULT_RESPONSE_HTML)),
            Err(_) => String::from("Error writing to file")
        }
    }
    
    async fn parse_json_response(&self) -> String {
        match serde_json::from_slice::<Value>(&self.body) {
            Ok(json) => serde_json::to_string_pretty(&json).unwrap_or_else(|_| "Error formatting JSON".into()),
            Err(_) => String::from("Error parsing JSON")
        }
    }

    fn get_content_type(&self) -> ContentType {
        let raw = self
            .headers
            .get(CONTENT_TYPE)
            .and_then(|hv| hv.to_str().ok())
            .map(str::to_string)
            .unwrap_or_default();

        ContentType::from_str(raw)
    }
}
