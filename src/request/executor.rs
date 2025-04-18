use std::error::Error;

use crate::response::model::ResponseWrapper;

use super::model::Request;

pub async fn perform_request(request: Request) -> Result<ResponseWrapper, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let mut builder = client.request(request.method.as_reqwest_method(), request.url.to_raw());

    println!("{:?}", request.headers);
    println!("{:?}", request.params);
    println!("{:?}", request.body);
    println!("{:?}", request.method);
    if let Some(body) = request.body {
        builder = builder.body(body);
    }

    let response = builder.send().await?;

    Ok(ResponseWrapper::from_response(response).await)
}

