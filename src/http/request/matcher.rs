use crate::http::request::{Request, RequestMethod};

pub struct RequestMatcher {
    method: RequestMethod,
    url: String,
}

pub struct RequestMatcherBuilder {
    method: RequestMethod,
    url: String,
}

impl RequestMatcher {
    fn new(builder: RequestMatcherBuilder) -> RequestMatcher {
        RequestMatcher {
            method: builder.method,
            url: builder.url,
        }
    }

    pub fn post() -> RequestMatcherBuilder {
        RequestMatcherBuilder::new(RequestMethod::POST)
    }

    pub fn get() -> RequestMatcherBuilder {
        RequestMatcherBuilder::new(RequestMethod::GET)
    }

    pub fn delete() -> RequestMatcherBuilder {
        RequestMatcherBuilder::new(RequestMethod::DELETE)
    }

    pub fn put() -> RequestMatcherBuilder {
        RequestMatcherBuilder::new(RequestMethod::PUT)
    }

    pub fn matches(&self, request: &Request) -> bool {
        self.method == request.method && self.url == request.url
    }
}

impl RequestMatcherBuilder {
    fn new(method: RequestMethod) -> RequestMatcherBuilder {
        RequestMatcherBuilder {
            method,
            url: String::from(""),
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Into::into(url);
        self
    }

    pub fn build(self) -> RequestMatcher {
        RequestMatcher::new(self)
    }
}

mod tests {
    use super::*;

    #[test]
    fn builder_must_construct_post_matcher_correctly() {
        let matcher = RequestMatcher::post().url("test").build();

        assert_eq!(
            RequestMethod::POST,
            matcher.method,
            "Request method must be 'POST'"
        );
        assert_eq!("test", matcher.url, "URL must be 'test'");
    }

    #[test]
    fn builder_must_construct_get_matcher_correctly() {
        let matcher = RequestMatcher::get().url("test").build();

        assert_eq!(
            RequestMethod::GET,
            matcher.method,
            "Request method must be 'GET'"
        );
        assert_eq!("test", matcher.url, "URL must be 'test'");
    }

    #[test]
    fn builder_must_construct_put_matcher_correctly() {
        let matcher = RequestMatcher::put().url("test").build();

        assert_eq!(
            RequestMethod::PUT,
            matcher.method,
            "Request method must be 'PUT'"
        );
        assert_eq!("test", matcher.url, "URL must be 'test'");
    }

    #[test]
    fn builder_must_construct_delete_matcher_correctly() {
        let matcher = RequestMatcher::delete().url("test").build();

        assert_eq!(
            RequestMethod::DELETE,
            matcher.method,
            "Request method must be 'DELTE'"
        );
        assert_eq!("test", matcher.url, "URL must be 'test'");
    }

    #[test]
    fn matcher_must_match_request() {
        let matcher = RequestMatcher::get().url("test").build();

        let request = Request::builder()
            .method(RequestMethod::GET)
            .url("test")
            .build();

        assert!(matcher.matches(&request));
    }
}
