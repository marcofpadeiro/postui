use response::model::{ResponseSubtype, ResponseType};
use response::parser::{parse_html_response, parse_json_response};

use crate::request::executor::perform_request;
use crate::request::parser::parse_file;
use std::env::args;

mod request;
mod config;
mod response;

#[tokio::main]
async fn main() {
    let args: Vec<String> = args().collect();
    let request = parse_file(args.get(1).unwrap()).unwrap();
    let response = perform_request(request).await.unwrap();

    let (response_type, response_subtype) =
        response::parser::parse_content_type(response.headers().get("Content-Type").unwrap());

    // Maybe split this into some function ¯\(ツ)/¯
    let content: String = match (response_type, response_subtype) {
        (ResponseType::Text, ResponseSubtype::Html) => parse_html_response(response).await,
        (ResponseType::Application, ResponseSubtype::Json) => parse_json_response(response).await,
        _ => response
            .text()
            .await
            .unwrap_or_else(|_| String::from("Error parsing response")),
    };

    println!("Response: {}", content);
}
