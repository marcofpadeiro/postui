use std::env::args;

use request::{parse_file, perform_request};

mod request;

#[tokio::main]
async fn main() {
    let args: Vec<String> = args().collect();
    let request = parse_file(args.get(1).unwrap()).unwrap();

    println!("{}", perform_request(request).await.unwrap().text().await.unwrap());
}
