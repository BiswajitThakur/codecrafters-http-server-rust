use std::{
    io::{self, Write},
    net::TcpListener,
};

use codecrafters_http_server::{Method, Request};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut req = Request::try_from(stream)?;
                match (req.method, req.target.as_ref()) {
                    (Method::Get, "/") => req.write_all(b"HTTP/1.1 200 OK\r\n\r\n")?,
                    (Method::Get, _) => req.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n")?,
                    _ => {}
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}
