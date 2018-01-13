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
    pub fn parse() -> Result<Request, &'static str> {

        /***** Retrieve CGI variables from the environment *****/

        let remote_addr = match env::var("REMOTE_ADDR") {
            Ok(val) => val,
            Err(_err) => return Err("Failed to retrieve REMOTE_ADDR"),
        };

        let request_method = match env::var("REQUEST_METHOD") {
            Ok(val) => match val.as_str() {
                "GET"  => RequestMethod::Get,
                "POST" => RequestMethod::Post,
                &_     => RequestMethod::Invalid,
            },
            Err(_err) => return Err("Failed to retrieve REQUEST_METHOD"),
        };

        let query_str = match env::var("QUERY_STRING") {
            Ok(val) => val,
            Err(_err) => return Err("Failed to retrieve QUERY_STRING"),
        };

        Ok(Request {
            remote_address: remote_addr,
            method: request_method,
            host: parse_host_from_query_string(&query_str),
            ip: parse_ip_from_query_string(&query_str),
            debug: query_str.contains("debug=true"),
        })
    }

}

fn parse_host_from_query_string(query_str: &str) -> String {
    // DNS label has a limit of 63 characters
    let host_re = regex::Regex::new(r"host=(\w{1,63})").unwrap();

    let host_str = host_re.captures(&query_str).map_or("", |caps| {
        caps.get(1).map_or("", |cap| cap.as_str())
    });

    String::from(host_str)
}

fn parse_ip_from_query_string(query_str: &str) -> String {
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

    String::from(ip_str)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_host() {
        assert_eq!("test", parse_host_from_query_string("host=test&ip=192.168.100.1&extraparam=garbage"));
    }

    #[test]
    fn valid_ipv4() {
        assert_eq!("192.168.100.1", parse_ip_from_query_string("host=test&ip=192.168.100.1&extraparam=garbage"));
    }

    #[test]
    fn valid_ipv6() {
        assert_eq!("fc00::1", parse_ip_from_query_string("host=test&ip=fc00::1&extraparam=garbage"));
    }

    #[test]
    fn invalid_host() {
        assert_ne!("test123.invalid", parse_host_from_query_string("host=test123.invalid"));
    }

    #[test]
    fn invalid_ipv4() {
        assert_eq!("", parse_ip_from_query_string("ip=999.168.100.1"));
    }

    #[test]
    fn invalid_ipv6() {
        assert_eq!("", parse_ip_from_query_string("ip=fc00:1.1.1.1:1"));
    }

    #[test]
    fn no_host() {
        assert_eq!("", parse_host_from_query_string("ip=1.1.1.1"));
    }

    #[test]
    fn no_ip() {
        assert_eq!("", parse_ip_from_query_string("host=test"));
    }
}
