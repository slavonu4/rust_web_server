use std::{
    collections::HashMap,
    fmt::Display,
    io::{Error, Write},
    net::TcpStream,
};

pub struct Response {
    code: u16,
    body: String,
    headers: HashMap<String, String>,
}

#[derive(Default)]
pub struct ResponseBuilder {
    code: u16,
    body: String,
    headers: HashMap<String, String>,
}

impl Response {
    pub fn builder() -> ResponseBuilder {
        ResponseBuilder::default()
    }

    fn new(builder: ResponseBuilder) -> Response {
        Response {
            code: builder.code,
            body: builder.body,
            headers: builder.headers,
        }
    }

    pub fn code(&self) -> u16 {
        self.code
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn write(self, stream: &mut TcpStream) -> Result<(), Error> {
        let response_string = self.to_string();
        stream.write_all(response_string.as_bytes())
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let headers = self
            .headers
            .iter()
            .fold(String::new(), |acc, (name, value)| {
                acc + name + ": " + value + "\r\n"
            });

        write!(f, "HTTP/1.1 {}\r\n{}\r\n{}", self.code, headers, self.body)
    }
}

impl ResponseBuilder {
    pub fn code(mut self, code: u16) -> ResponseBuilder {
        self.code = code;

        self
    }

    pub fn body(mut self, body: impl Into<String>) -> ResponseBuilder {
        self.body = Into::into(body);

        self
    }

    pub fn add_header(
        mut self,
        header_name: impl Into<String>,
        header_value: impl Into<String>,
    ) -> ResponseBuilder {
        self.headers
            .insert(Into::into(header_name), Into::into(header_value));

        self
    }

    pub fn build(self) -> Response {
        Response::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_must_construct_response_correctly() {
        let response = Response::builder().code(200).body("test_body").build();

        assert_eq!(200, response.code(), "Response code must be 200");
        assert_eq!(
            "test_body",
            response.body(),
            "Response body must be 'test_body"
        );
    }
}
