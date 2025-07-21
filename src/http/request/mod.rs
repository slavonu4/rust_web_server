use std::collections::HashMap;

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

impl Request {
    fn new(builder: RequestBuilder) -> Request {
        Request {
            url: builder.url,
            method: builder.method,
            headers: builder.headers,
            query_params: builder.query_params,
        }
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
        let header_value = Into::into(header_value);

        self.headers
            .entry(Into::into(header_name))
            .and_modify(|v| v.push(header_value.clone()))
            .or_insert(vec![header_value.clone()]);

        self
    }

    pub fn add_query_param(
        mut self,
        query_param_name: impl Into<String>,
        query_param_value: impl Into<String>,
    ) -> Self {
        let query_param_value = Into::into(query_param_value);

        self.query_params
            .entry(Into::into(query_param_name))
            .and_modify(|v| v.push(query_param_value.clone()))
            .or_insert(vec![query_param_value.clone()]);

        self
    }

    pub fn build(self) -> Request {
        Request::new(self)
    }

    // TODO: implement query params
}

mod tests {
    use super::*;

    #[test]
    fn builder_must_build_request() {
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
}
