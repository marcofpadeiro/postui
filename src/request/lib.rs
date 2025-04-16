use std::collections::HashMap;

enum RequestMethod {
    GET,
    POST,
    DELETE,
    PUT,
    OPTION
}

struct Request {
    name: String,
    description: String,
    method: RequestMethod,
    url: String,
    body: String,
    headers: HashMap<String, String>,
    params: HashMap<String, String>
}
