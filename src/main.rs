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

    let content = response.parse_response().await;

    println!("Response: {}", content);
}
