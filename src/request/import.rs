use std::{collections::HashMap, error::Error, fs::read_to_string, path::Path};

use serde_json::Value;

use crate::{config::get_requests_dir_path, request::model::{Request, RequestMethod}};

use super::parser::write_into_file;

pub fn import(file_path: &String) -> Result<(), Box<dyn Error>> {
    let path = Path::new(file_path);
    let json = read_postman_export(path)?;
    let requests = map_to_postui_requests(json)?;
    let out_dir = get_requests_dir_path()?;

    for request in requests {
        let filename = determine_file_name_from_request(&request);
        let dest = out_dir.join(&filename);

        println!("{:?}", dest);
    
        if let Err(e) = write_into_file(&request, dest) {
            eprintln!("Error writing request {}: {}", request.name, e);
            continue;
        }
    
        eprintln!("Imported {} successfully", request.name);
    }
    Ok(())
}

fn read_postman_export<P: AsRef<Path>>(file: P) -> Result<Value, Box<dyn Error>> {
    let json = read_to_string(file)?;

    let parse: Value = serde_json::from_str(&json)?;

    Ok(parse)
}

fn map_to_postui_requests(json: Value) -> Result<Vec<Request>, Box<dyn Error>> {
    let arr = match json {
        Value::Array(a) => a,
        _ => return Err("JSON must be an array".into()),
    };

    let mut requests = Vec::with_capacity(arr.len());

    for entry in arr {
        let mut item: Request = Default::default();
        if let Some(name) = entry.get("name").and_then(Value::as_str) {
            item.name = name.to_string();
        }
        if let Some(request) = entry.get("request") {
            if let Some(method) = request.get("method").and_then(Value::as_str) {
                item.method =  RequestMethod::from_str(&method);
            }
            if let Some(headers) = request.get("header") {
                item.headers = HashMap::new();
                for i in headers.as_array().unwrap() {
                    let key = i.get("key").and_then(Value::as_str).unwrap().to_string();
                    let value = i.get("value").and_then(Value::as_str).unwrap().to_string();
                    item.headers.insert(key, value);
                }
            }
            if let Some(url) = request.get("url") {
                if let Some(host) = url.get("host") {
                    item.url.host = host.as_array().unwrap().iter().map(|x| x.to_string()).next().unwrap().trim_matches('"').to_string();
                }

                if let Some(path) = url.get("path") {
                    item.url.path = path.as_array().unwrap().iter().map(|x| x.to_string().trim_matches('"').to_string()).collect();
                }

                if let Some(params) = url.get("query") {
                    for i in params.as_array().unwrap() {
                        let key = i.get("key").and_then(Value::as_str).unwrap().to_string();
                        let value = i.get("value").and_then(Value::as_str).unwrap().to_string();
                        item.params.insert(key, value);
                    }
                }
            }
            if let Some(body) = request.get("body") {
                item.body = body.get("raw").and_then(Value::as_str).map(|x| x.to_string());
            }
        }
        requests.push(item);
    }

    Ok(requests)
}

fn determine_file_name_from_request(request: &Request) -> String {
    let name = request.name.trim().to_lowercase();

    let mut filename = String::with_capacity(name.len());
    let mut last_was_dash = false;

    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            filename.push(c);
            last_was_dash = false;
        } else if !last_was_dash {
            filename.push('-');
            last_was_dash = true;
        }
    }

    let filename = filename.trim_matches('-').to_string();

    format!("{}.toml", filename)
}

