use std::env::args;
use ui::tui::Tui;

use crate::request::import::import;

mod config;
mod request;
mod response;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = args().collect();
    // let request = parse_file(args.get(1).unwrap()).unwrap();
    // let response = perform_request(request).await.unwrap();
    //
    // let content = response.parse_response().await;

    // HINT: Run tui
    // let terminal = ratatui::init();
    // let result = Tui::new().run(terminal);
    // ratatui::restore();
    // result
    
    import(args.get(1).unwrap())
}
