use http::server::Server;

use http::server::Config;

use crate::http::request::matcher::RequestMatcher;
use crate::http::response::Response;

pub mod concurrent;
pub mod http;

fn main() {
    let config = Config::get_config();
    let server = Server::builder(config)
        .register_handler(RequestMatcher::get().url("/test").build(), |_| {
            Response::builder().code(200).body("Test").build()
        })
        .build();
    server.start();
}
