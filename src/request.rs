extern crate regex;

use std::env;
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug)]
pub enum RequestMethod {
    Invalid,
    Get,
    Post,
}

impl Default for RequestMethod {
    fn default() -> RequestMethod { RequestMethod::Invalid }
}


#[derive(Default, Debug)]
pub struct Request {
    pub remote_address: String,
    pub method: RequestMethod,
    pub host: String,
    pub ip: String,
    pub debug: bool,
}

impl Request {
    pub fn parse() -> Option<Request> {

        /***** Retrieve CGI variables from the environment *****/

        let remote_addr = match env::var("REMOTE_ADDR") {
            Ok(val) => val,
            Err(_err) => {
                eprintln!("Failed to retrieve REMOTE_ADDR: {}", _err);
                return None
            },
        };

        let request_method = match env::var("REQUEST_METHOD") {
            Ok(val) => match val.as_str() {
                "GET"  => RequestMethod::Get,
                "POST" => RequestMethod::Post,
                &_     => RequestMethod::Invalid,
            },
            Err(_err) => {
                eprintln!("Failed to retrieve REQUEST_METHOD: {}", _err);
                return None
            },
        };

        let query_str = match env::var("QUERY_STRING") {
            Ok(val) => val,
            Err(_err) => {
                eprintln!("Failed to retrieve QUERY_STRING: {}", _err);
                return None
            },
        };

        /***** Process the query *****/

        // DNS label has a limit of 63 characters
        let host_re = regex::Regex::new(r"host=(\w{1,63})").unwrap();
        let host_str = host_re.captures(&query_str).map_or("", |caps| {
            caps.get(1).map_or("", |cap| cap.as_str())
        });

        // IP address could be IPv4 or IPv6
        let ip_re = regex::Regex::new(r"ip=([0-9a-fA-F.:]{1,45})").unwrap();
        let ip_str = ip_re.captures(&query_str).map_or("", |caps| {
            caps.get(1).map_or("", |cap| {
                // Try to create an IpAddr object to validate the value
                match IpAddr::from_str(cap.as_str()) {
                    Ok(_val) => cap.as_str(),
                    Err(_error) => "",
                }
            })
        });

        let debug = query_str.contains("debug=true");

        Some(Request {
            remote_address: String::from(remote_addr),
            method: request_method,
            host: String::from(host_str),
            ip: String::from(ip_str),
            debug: debug,
        })
    }
}
