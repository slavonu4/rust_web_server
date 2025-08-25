use clap::Parser;
use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener},
    ops::RangeInclusive,
    sync::Arc,
};

use crate::{
    concurrent::thread_pool::ThreadPool,
    http::{
        request::{matcher::RequestMatcher, Request},
        response::Response,
    },
};

pub type HandlerFn = Box<dyn Fn(Request) -> Response + Send + Sync + 'static>;

struct RequestHandler {
    matcher: RequestMatcher,
    handler_fn: HandlerFn,
}

pub struct Server {
    pool: ThreadPool,
    address: SocketAddr,
    handlers: Arc<Vec<RequestHandler>>,
}

pub struct ServerBuilder {
    pool_size: usize,
    host: Ipv4Addr,
    port: u16,
    handlers: Vec<RequestHandler>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(long, value_parser = valid_pool_size, default_value_t = 1)]
    pub pool_size: usize,
    #[arg(long, default_value = "127.0.0.1", value_parser = valid_address)]
    pub host: Ipv4Addr,
    #[arg(short, long, default_value_t = 8080, value_parser = port_in_range)]
    pub port: u16,
}

fn valid_pool_size(s: &str) -> Result<usize, String> {
    let pool_size: usize = s
        .parse()
        .map_err(|_| format!("{s} is not a valid pool size"))?;

    if pool_size > 0 {
        Ok(pool_size)
    } else {
        Err("Pool size can not be less than 1".to_string())
    }
}

fn valid_address(s: &str) -> Result<Ipv4Addr, String> {
    s.parse()
        .map_err(|_| format!("{s} is not a valid IPv4 string"))
}

const PORT_RANGE: RangeInclusive<u16> = 1..=65535;

fn port_in_range(s: &str) -> Result<u16, String> {
    let port: u16 = s
        .parse()
        .map_err(|_| format!("{s} is not a valid port number"))?;

    if PORT_RANGE.contains(&port) {
        Ok(port)
    } else {
        Err(format!(
            "port is not range [{} - {}]",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}

impl Server {
    fn new(builder: ServerBuilder) -> Server {
        let thread_pool = ThreadPool::new(builder.pool_size);
        let address = SocketAddrV4::new(builder.host, builder.port);

        Server {
            pool: thread_pool,
            address: SocketAddr::V4(address),
            handlers: Arc::new(builder.handlers),
        }
    }

    pub fn builder(config: Config) -> ServerBuilder {
        ServerBuilder {
            pool_size: config.pool_size,
            host: config.host,
            port: config.port,
            handlers: Vec::new(),
        }
    }

    pub fn start(&self) {
        println!(
            "Server is listening at {} (pool size={})",
            self.address,
            self.pool.size()
        );
        let listener = TcpListener::bind(self.address).unwrap();

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            let thread_handlers = Arc::clone(&self.handlers);
            self.pool.execute(move || {
                let request = Request::parse(&mut stream);

                let response = match request {
                    Ok(request) => {
                        let handler = thread_handlers.iter().find(|h| h.matcher.matches(&request));
                        match handler {
                            Some(handler) => (handler.handler_fn)(request),
                            None => not_found_response(),
                        }
                    }
                    Err(e) => server_error_response(e),
                };

                response.write(&mut stream).unwrap();
            });
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        println!("Server is shutting down");
    }
}

impl Config {
    pub fn get_config() -> Config {
        Config::parse()
    }
}

impl ServerBuilder {
    pub fn pool_size(mut self, pool_size: usize) -> ServerBuilder {
        self.pool_size = pool_size;

        self
    }

    pub fn port(mut self, port: u16) -> ServerBuilder {
        self.port = port;

        self
    }

    pub fn host(mut self, host: Ipv4Addr) -> ServerBuilder {
        self.host = host;

        self
    }

    pub fn register_handler(
        mut self,
        request_matcher: RequestMatcher,
        request_handler: impl Fn(Request) -> Response + Send + Sync + 'static,
    ) -> ServerBuilder {
        let handler = RequestHandler {
            matcher: request_matcher,
            handler_fn: Box::new(request_handler),
        };

        self.handlers.push(handler);

        self
    }

    pub fn build(self) -> Server {
        Server::new(self)
    }
}

fn not_found_response() -> Response {
    Response::builder()
        .code(404)
        .body("Requested page has not been found")
        .build()
}

fn server_error_response<E>(error: E) -> Response
where
    E: Error,
{
    let response_body = format!("Something went wrong: {}", error);

    Response::builder().code(500).body(response_body).build()
}
