use clap::Parser;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream},
    ops::RangeInclusive,
    thread,
    time::Duration,
};

use crate::concurrent::thread_pool::ThreadPool;

pub struct Server {
    pool: ThreadPool,
    address: SocketAddr,
}

pub struct ServerBuilder {
    pool_size: usize,
    host: Ipv4Addr,
    port: u16,
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
        }
    }

    pub fn builder(config: Config) -> ServerBuilder {
        ServerBuilder {
            pool_size: config.pool_size,
            host: config.host,
            port: config.port,
        }
    }

    pub fn start(&self) {
        println!(
            "Server is listening at {} (pool size={})",
            self.address,
            self.pool.size()
        );
        let listener = TcpListener::bind(self.address).unwrap();
        //TODO: implement parsing incoming request and using registered handles to process it
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            self.pool.execute(|| handle_connection(stream));
        }
    }

    //TODO: implement registering request matchers with request handlers
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

    pub fn build(self) -> Server {
        Server::new(self)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&stream);

    let request_line = reader.lines().next().unwrap().unwrap();
    let request_line = request_line.trim();

    let (status_line, file_name) = match request_line {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let content = fs::read_to_string(file_name).unwrap();
    let content_length = content.len();

    let response = format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n {content}");
    stream.write_all(response.as_bytes()).unwrap();
}
