use http;
use std::env;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ip_address = extract_ip_address_argument(&args);
    let listener = http::Server::bind(ip_address);

    listener.listen(
        core::controller::RequestController::new("./public".to_string())
    ).unwrap();
}

fn extract_ip_address_argument(args: &Vec<String>) -> String {
    if args.len() != 2 {
        panic!("Please pass only the address with the port! Example 127.0.0.1:80");
    }

    let ip_address = args.get(1).unwrap().as_str().to_string();

    if !validate_ip_address(&ip_address) {
        panic!("Ip address is not valid!");
    }

    ip_address
}

fn validate_ip_address(ip_address: &str) -> bool {
    let regex_template = Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d{1,5}$").unwrap();

    regex_template.is_match(ip_address)
}
