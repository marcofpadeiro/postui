use std::env::args;

use request::parse_file;

mod request;

fn main() {
    let args: Vec<String> = args().collect();
    let res = parse_file(args.get(1).unwrap());

    println!("{:?}", res);
}
