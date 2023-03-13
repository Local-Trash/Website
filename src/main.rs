use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream}, ops::Deref,
};
mod threadpool;
use threadpool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(10);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];

    stream.read(&mut buffer).unwrap();

    let pages = [
        Page::new("", include_str!("html/index.html"))
    ];

    let mut response = false;
    for page in &pages {
        if buffer.starts_with(&page.url[..]) {
            stream.write(&page.content).unwrap();
            response = true;
        }
    }
    if !response {
        stream.write(&pages[0].content).unwrap();
    }

    stream.flush().unwrap();
}

struct Page {
    url: Vec<u8>,
    content: Vec<u8>
}

impl Page {
    fn new(url: &str, content: &str) -> Self {
        Self {
            url: format!("GET /{}", url).as_bytes().to_vec(),
            content: format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", content.len(), content).as_bytes().to_vec(),
        }
    }
}

impl Deref for Page {
    type Target = Page;

    fn deref(&self) -> &Self::Target {
        &self
    }
}