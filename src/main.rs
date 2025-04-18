use std::env::args;

use crate::request::import::import;

mod request;
mod config;
mod response;

#[tokio::main]
async fn main() {
    let args: Vec<String> = args().collect();
    // let request = parse_file(args.get(1).unwrap()).unwrap();
    // let response = perform_request(request).await.unwrap();
    //
    // let content = response.parse_response().await;

    import(args.get(1).unwrap()).unwrap();
}
