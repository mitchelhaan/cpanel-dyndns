
#[derive(Default)]
pub struct Response {
    headers: Vec<String>,
    content: Vec<String>,
    status_code: u16,
}

impl Response {
    pub fn new() -> Response {
        Default::default()
    }

    pub fn add_header(&mut self, header: &str) {
        self.headers.push(header.to_string())
    }

    pub fn add_content(&mut self, content: &str) {
        self.content.push(content.to_string())
    }

    pub fn set_status_code(&mut self, status_code: u16) {
        if self.status_code == 0 {
            let (status_code, status_text) = Response::status_code_text(status_code);
            self.headers.push(format!("Status: {} {}", status_code, status_text));
            self.status_code = status_code;
        } else {
            eprintln!("Status code has already been set to {}! Ignoring.", self.status_code);
        }
    }

    pub fn send(self) {
        for h in self.headers {
            println!("{}", h);
        }
        println!("");

        for c in self.content {
            println!("{}", c);
        }
    }

    fn status_code_text(status_code: u16) -> (u16, &'static str) {
        let mut return_status_code = status_code;
        let status_text = match status_code {
            200 => "OK",
            201 => "Created",
            202 => "Accepted",
            204 => "No Content",
            304 => "Not Modified",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            501 => "Not Implemented",
            503 => "Service Unavailable",
            _   => {return_status_code = 500; "Internal Server Error"},
        };

        (return_status_code, status_text)
    }
}
