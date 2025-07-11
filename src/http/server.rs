use clap::Parser;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use crate::concurrent::thread_pool::ThreadPool;

pub struct Server {
    pool: ThreadPool,
    address: String,
}

// TODO: see if some validation and default value for address can be added
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(short, long, default_value_t = 1)]
    pub pool_size: usize,
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    pub address: String,
}

impl Server {
    pub fn create(config: Config) -> Server {
        let thread_pool = ThreadPool::new(config.pool_size);

        Server {
            pool: thread_pool,
            address: config.address,
        }
    }

    pub fn start(&self) {
        println!(
            "Server is listening at {} (pool size={})",
            self.address,
            self.pool.size()
        );
        let listener = TcpListener::bind(&self.address).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            self.pool.execute(|| handle_connection(stream));
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
