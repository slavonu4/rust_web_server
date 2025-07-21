pub struct Response {
    code: u16,
    body: String,
}

#[derive(Default)]
pub struct ResponseBuilder {
    code: u16,
    body: String,
}

impl Response {
    pub fn builder() -> ResponseBuilder {
        ResponseBuilder::default()
    }

    fn new(builder: ResponseBuilder) -> Response {
        Response {
            code: builder.code,
            body: builder.body,
        }
    }

    pub fn code(&self) -> u16 {
        self.code
    }

    pub fn body(&self) -> &str {
        &self.body
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

    pub fn build(self) -> Response {
        Response::new(self)
    }
}

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
