use std::{collections::HashMap, fmt::format, io::Error};

pub mod matcher;

#[derive(Debug, PartialEq)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

pub struct Request {
    url: String,
    method: RequestMethod,
    headers: HashMap<String, Vec<String>>,
    query_params: HashMap<String, Vec<String>>,
}

pub struct RequestBuilder {
    url: String,
    method: RequestMethod,
    headers: HashMap<String, Vec<String>>,
    query_params: HashMap<String, Vec<String>>,
}

impl RequestMethod {
    pub fn parse(method: &str) -> Option<RequestMethod> {
        match method {
            "GET" => Some(RequestMethod::GET),
            "POST" => Some(RequestMethod::POST),
            "PUT" => Some(RequestMethod::PUT),
            "DELETE" => Some(RequestMethod::DELETE),
            _ => None,
        }
    }
}

impl Request {
    fn new(builder: RequestBuilder) -> Request {
        Request {
            url: builder.url,
            method: builder.method,
            headers: builder.headers,
            query_params: builder.query_params,
        }
    }

    pub fn parse(request_str: &str) -> Result<Request, Error> {
        let parser_error = parser_error(String::from("Unable to parse an incoming request"));
        let mut parts = request_str.split("\r\n");

        let (method, path) = match parts.next() {
            Some(request_line) => parse_request_line(request_line)?,
            None => return Err(parser_error),
        };
        let (url, query_params) = parse_path(&path)?;
    }

    pub fn builder() -> RequestBuilder {
        RequestBuilder {
            url: String::from(""),
            method: RequestMethod::GET,
            headers: HashMap::new(),
            query_params: HashMap::new(),
        }
    }

    pub fn get_header(&self, header_name: &str) -> Option<&Vec<String>> {
        self.headers.get(header_name)
    }

    pub fn get_query_param(&self, query_param_name: &str) -> Option<&Vec<String>> {
        self.query_params.get(query_param_name)
    }
}

fn parse_request_line(request_line: &str) -> Result<(RequestMethod, String), Error> {
    let parse_error = parser_error(format!("Invalid request line: {}", request_line));

    let mut request_line_parts = request_line.split(" ");

    let request_method = match request_line_parts.next() {
        Some(request_method_str) => RequestMethod::parse(request_method_str),
        None => return Err(parse_error),
    };
    let request_method = match request_method {
        Some(request_method) => request_method,
        None => return Err(parse_error),
    };

    let path = match request_line_parts.next() {
        Some(path) => String::from(path),
        None => return Err(parse_error),
    };

    Ok((request_method, path))
}

fn parse_path(path: &str) -> Result<(String, HashMap<String, Vec<String>>), Error> {
    let parse_error = parser_error(format!("Invalid path: {}", path));

    let mut path_parts = path.split("&");

    let url = match path_parts.next() {
        Some(url) => String::from(url),
        None => return Err(parse_error),
    };

    let query_params = match path_parts.next() {
        Some("") => HashMap::default(),
        Some(query_str) => parse_query_params(query_str)?,
        None => HashMap::default(),
    };

    Ok((url, query_params))
}

fn parse_query_params(query_str: &str) -> Result<HashMap<String, Vec<String>>, Error> {
    let query_params = query_str.split("&");

    let mut result: HashMap<String, Vec<String>> = HashMap::new();
    for query_param in query_params {
        let (param_name, param_values) = parse_query_param(query_param)?;

        let existing_param_values = result.entry(param_name).or_default();

        existing_param_values.extend(param_values);
    }

    Ok(result)
}

fn parse_query_param(query_param: &str) -> Result<(String, Vec<String>), Error> {
    let parse_error = parser_error(format!("Invalid query param: {}", query_param));
    let mut query_param_parts = query_param.split("=");
    let param_name = match query_param_parts.next() {
        Some(param_name) => String::from(param_name),
        None => return Err(parse_error),
    };
    let param_values = match query_param_parts.next() {
        Some(param_values) => parse_query_param_values(param_values)?,
        None => return Err(parse_error),
    };

    Ok((param_name, param_values))
}

fn parse_query_param_values(query_param_values: &str) -> Result<Vec<String>, Error> {
    let parse_error = parser_error(format!("Invalid query param value: {}", query_param_values));

    let values: Vec<String> = query_param_values.split(",").map(String::from).collect();

    if values.is_empty() {
        Err(parse_error)
    } else {
        Ok(values)
    }
}

fn parser_error(error_message: String) -> Error {
    Error::new(std::io::ErrorKind::InvalidData, error_message)
}

impl RequestBuilder {
    pub fn method(mut self, method: RequestMethod) -> Self {
        self.method = method;
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Into::into(url);
        self
    }

    pub fn add_header(
        mut self,
        header_name: impl Into<String>,
        header_value: impl Into<String>,
    ) -> Self {
        let header_name = Into::into(header_name);
        let header_value = Into::into(header_value);

        let header_entry = self.headers.entry(header_name).or_default();
        header_entry.push(header_value);

        self
    }

    pub fn add_query_param(
        mut self,
        query_param_name: impl Into<String>,
        query_param_value: impl Into<String>,
    ) -> Self {
        let query_param_name = Into::into(query_param_name);
        let query_param_value = Into::into(query_param_value);

        let query_param_entry = self.query_params.entry(query_param_name).or_default();
        query_param_entry.push(query_param_value);

        self
    }

    pub fn build(self) -> Request {
        Request::new(self)
    }
}

mod tests {
    use super::*;

    #[test]
    fn builder_must_construct_request_correctly() {
        let request = Request::builder()
            .method(RequestMethod::GET)
            .url("test")
            .add_header("test_header_1", "test_value_1_1")
            .add_header("test_header_1", "test_value_1_2")
            .add_header("test_header_2", "test_value_2_1")
            .add_query_param("test_qp_1", "test_qp_value_1_1")
            .add_query_param("test_qp_1", "test_qp_value_1_2")
            .add_query_param("test_qp_2", "test_qp_value_2_1")
            .build();

        assert_eq!(
            RequestMethod::GET,
            request.method,
            "Request method must be 'GET'"
        );
        assert_eq!("test", request.url, "Request url must be 'test'");

        let header_1 = request.get_header("test_header_1");
        assert!(
            header_1.is_some_and(|h| h.len() == 2 && h.contains(&String::from("test_value_1_1")) && h.contains(&String::from("test_value_1_2"))),
            "Header 'test_header_1' must be present and contain 2 values('test_value_1_1' and 'test_value_1_2')"
        );

        let header_2 = request.get_header("test_header_2");
        assert!(
            header_2.is_some_and(|h| h.len() == 1 && h.contains(&String::from("test_value_2_1"))),
            "Header 'test_header_2' must be present and contain only 1 value('test_value_2_1')"
        );

        let header_3 = request.get_header("test_header_3");
        assert!(
            header_3.is_none(),
            "Request must not have header 'test_header_3'"
        );

        let query_param_1 = request.get_query_param("test_qp_1");
        println!("{}", query_param_1.is_some());
        assert!(
            query_param_1.is_some_and(|qp| qp.len() == 2 && qp.contains(&String::from("test_qp_value_1_1")) && qp.contains(&String::from("test_qp_value_1_2"))),
            "Query param 'test_qp_1' must be present and contain 2 values('test_qp_value_1_1' and 'test_qp_value_1_2')"
        );

        let query_param_2 = request.get_query_param("test_qp_2");
        assert!(
            query_param_2
                .is_some_and(|qp| qp.len() == 1 && qp.contains(&String::from("test_qp_value_2_1"))),
            "Query param 'test_qp_2' must be present and contain only 1 value('test_qp_value_2_1')"
        );

        let query_param_3 = request.get_query_param("test_qp_3");
        assert!(
            query_param_3.is_none(),
            "Request must not have header 'test_qp_3'"
        );
    }

    #[test]
    fn request_method_parse_must_return_correct_value() {
        let get = RequestMethod::parse("GET");
        let post = RequestMethod::parse("POST");
        let put = RequestMethod::parse("PUT");
        let delete = RequestMethod::parse("DELETE");
        let unknown = RequestMethod::parse("unknown");

        assert!(
            get.is_some_and(|m| m == RequestMethod::GET),
            "GET method must be parsed correctly"
        );
        assert!(
            post.is_some_and(|m| m == RequestMethod::POST),
            "POST method must be parsed correctly"
        );
        assert!(
            put.is_some_and(|m| m == RequestMethod::PUT),
            "PUT method must be parsed correctly"
        );
        assert!(
            delete.is_some_and(|m| m == RequestMethod::DELETE),
            "DELETE method must be parsed correctly"
        );
        assert!(unknown.is_none(), "Unknown method must be parsed into None");
    }
}
