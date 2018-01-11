mod request;
use request::{Request, RequestMethod};
mod response;
use response::Response;
mod cpanel;
mod host;
mod database;

const DB_FILE: &str = "dyndns.db";


fn main() {
    let mut response = Response::new();

    // For now, assume we'll always be returning plain text
    response.add_header("Content-Type: text/plain");

    let request = match Request::parse() {
        Some(val) => val,
        None => {
            response.set_status_code(500);
            response.send();
            return
        },
    };

    let db = database::open(DB_FILE);

    if request.debug {
        response.add_content(&format!("{:#?}", request));
    }

    match request.method {
        RequestMethod::Get => {
            let existing_host = host::read(&db, &request.host);
            response.set_status_code(200);
            response.add_content(&format!("{:#?}", existing_host));
        },

        RequestMethod::Post => {
            // Use the supplied IP address if available, fall back to the client IP
            let new_ip = if request.ip != "" {
                request.ip
            } else {
                request.remote_address
            };

            let updated_host = host::update(&db, &request.host, &new_ip);
            response.set_status_code(200);
            response.add_content(&format!("{:#?}", updated_host));
        },

        _ => {
            // Bad request
            response.set_status_code(400);
        },
    }

    response.send();
}
