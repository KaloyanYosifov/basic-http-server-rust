use http;
use std::env;
use regex::Regex;
use std::collections::HashMap;
use config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut settings = load_settings(&args);

    if let Ok(ip_address) = settings.get_str("ip_address") {
        let listener = http::Server::bind(ip_address);

        listener.listen(
            core::controller::RequestController::new(settings.get_str("public_path").unwrap())
        ).unwrap();
    } else {
        panic!("Please pass ip address directly or through config file!");
    }
}

fn extract_ip_address_argument(args: &Vec<String>) -> Option<String> {
    if args.len() != 2 {
        return None;
    }

    let ip_address = args.get(1).unwrap().as_str().to_string();

    if !validate_ip_address(&ip_address) {
        panic!("Ip address is not valid!");
    }

    Some(ip_address)
}

fn load_settings(args: &Vec<String>) -> Config {
    let mut settings = config::Config::new();

    settings
        .merge(config::File::with_name("config.json").required(false)).unwrap();

    if let Some(ip_address) = extract_ip_address_argument(&args) {
        settings.set("ip_address", ip_address).unwrap();
    }

    if let Err(_) = settings.get_str("public_path") {
        settings.set("public_path", "./public");
    }

    settings
}

fn validate_ip_address(ip_address: &str) -> bool {
    let regex_template = Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d{1,5}$").unwrap();

    regex_template.is_match(ip_address)
}
