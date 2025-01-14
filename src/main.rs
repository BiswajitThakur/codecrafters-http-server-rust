use std::{
    collections::HashMap,
    fs,
    io::{self, BufReader, Cursor, ErrorKind},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    thread,
};

use codecrafters_http_server::{Method, Request, Response, Status};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handler(stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}

fn handler(stream: TcpStream) -> io::Result<()> {
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
                    .content_length(paths[1].len() as u64)
                    .body(Cursor::new(paths[1]))
                    .send_to(req)?,
                "user-agent" => {
                    let body = req.get("User-Agent").unwrap().clone();
                    let body = body.as_ref();
                    default_res()
                        .status(Status::OK)
                        .content_type("text/plain")
                        .content_length(body.len() as u64)
                        .body(Cursor::new(body))
                        .send_to(req)?;
                }
                "files" => {
                    let args: Vec<String> = std::env::args().collect();
                    let args = parse_args(args);
                    let path: PathBuf = (&paths[1..]).iter().collect();
                    let path: PathBuf = [PathBuf::from(args.get("--directory").unwrap()), path]
                        .iter()
                        .collect();
                    let file = match fs::File::open(path) {
                        Ok(f) => f,
                        Err(e) if e.kind() == ErrorKind::NotFound => {
                            return default_res().send_to(req);
                        }
                        Err(e) => return Err(e),
                    };
                    let len = file.metadata()?.len();
                    let reader = BufReader::new(file);
                    Response::<BufReader<fs::File>>::default()
                        .status(Status::OK)
                        .content_type("application/octet-stream")
                        .content_length(len)
                        .body(reader)
                        .send_to(req)?;
                }
                _ => {
                    default_res().send_to(req)?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn default_res<'a>() -> Response<'a, Cursor<&'a str>> {
    Response::<Cursor<&str>>::default()
}

fn parse_args(args: Vec<String>) -> HashMap<&'static str, String> {
    let mut result = HashMap::new();
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--directory" => {
                result.insert("--directory", args.next().unwrap());
            }
            _ => {}
        }
    }
    result
}
