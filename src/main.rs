use crate::request::executor::perform_request;
use crate::request::parser::parse_file;
use std::env::args;

mod request;

#[tokio::main]
async fn main() {
    let args: Vec<String> = args().collect();
    let request = parse_file(args.get(1).unwrap()).unwrap();

    println!("{}", perform_request(request).await.unwrap().text().await.unwrap());
}
