use std::error::Error;

use reqwest::Response;

use super::model::Request;

pub async fn perform_request(request: Request) -> Result<Response, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let builder = client.request(request.method.as_reqwest_method(), request.url);

    let response = builder.send().await?;

    Ok(response)
}

