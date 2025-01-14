use std::{
    io::{self, Cursor},
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
                    (Method::Get, v) => {
                        let paths: Vec<&str> = v.split('/').filter(|v| !v.is_empty()).collect();
                        if paths.is_empty() {
                            return default_res().status(Status::OK).send_to(req);
                        }
                        match paths[0] {
                            "echo" => default_res()
                                .status(Status::OK)
                                .content_type("text/plain")
                                .content_length(paths[1].len())
                                .body(Cursor::new(paths[1]))
                                .send_to(req)?,
                            "user-agent" => {
                                let body = req.get("User-Agent").unwrap().clone();
                                let body = body.as_ref();
                                default_res()
                                    .status(Status::OK)
                                    .content_type("text/plain")
                                    .content_length(body.len())
                                    .body(Cursor::new(body))
                                    .send_to(req)?;
                            }
                            _ => {
                                default_res().send_to(req)?;
                            }
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

fn default_res<'a>() -> Response<'a, Cursor<&'a str>> {
    Response::<Cursor<&str>>::default()
}
