use std::{
    io::{self, Cursor, Write},
    net::TcpListener,
};

use codecrafters_http_server::{Method, Request, Response, Status};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let req = Request::try_from(stream)?;
                match (req.method, req.target.clone().as_ref()) {
                    (Method::Get, "/") => Response::<Cursor<&str>>::default()
                        .status(Status::OK)
                        .send_to(req)?,
                    (Method::Get, v) => {
                        let value = v.strip_prefix("/echo/");
                        if let Some(val) = value {
                            let mut res = Response::<Cursor<&str>>::default()
                                .status(Status::OK)
                                .content_type("text/plain")
                                .content_length(val.len())
                                .body(Cursor::new(val));
                            res.send_to(req)?;
                        } else {
                            Response::<Cursor<&str>>::default()
                                .status(Status::NotFound)
                                .send_to(req)?;
                        }
                    }
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
