use http::server::Server;

use http::server::Config;

pub mod concurrent;
pub mod http;

fn main() {
    let config = Config::get_config();
    let server = Server::builder(config).build();
    server.start();
}
