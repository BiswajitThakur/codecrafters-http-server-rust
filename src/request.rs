#![allow(unused_assignments)]

use std::{
    borrow::Cow,
    collections::HashMap,
    io::{self, ErrorKind, Read},
    net::TcpStream,
};

#[derive(Debug, Clone, Copy)]
pub enum Method {
    Connect,
    Delete,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
    Trace,
}

pub struct Request<'a> {
    pub method: Method,
    pub target: Cow<'a, str>,
    pub version: (u8, u8, u8),
    headers: HashMap<Cow<'a, str>, Cow<'a, str>>,
    stream: TcpStream,
}

impl<'a> Request<'a> {
    pub fn get<T: AsRef<str> + ?Sized>(&self, value: &'a T) -> Option<&Cow<'a, str>> {
        let value = value.as_ref();
        self.headers.get(&Cow::Borrowed(value))
    }
}

impl<'a> io::Read for Request<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf)
    }
}

impl<'a> io::Write for Request<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stream.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
    }
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.stream.write_all(buf)
    }
}

impl<'a> TryFrom<TcpStream> for Request<'a> {
    type Error = io::Error;
    fn try_from(mut stream: TcpStream) -> Result<Self, Self::Error> {
        let mut method = Method::Get;
        let mut target: Cow<'_, str> = Cow::default();
        let mut version = (0, 0, 0);
        let mut value = String::new();
        let mut buffer = [0; 1024];
        loop {
            let r = stream.read(&mut buffer)?;
            value.push_str(unsafe { std::str::from_utf8_unchecked(&buffer[0..r]) });
            if r == 0 || value.ends_with("\r\n\r\n") {
                break;
            }
        }
        let mut lines = value.lines();
        if let Some(line) = lines.next() {
            let vals: Vec<_> = line.split_whitespace().collect();
            if vals.len() != 3 {
                return Err(io::Error::new(ErrorKind::InvalidData, "Invalid Request"));
            }
            method = match vals[0].to_lowercase().as_str() {
                "connect" => Method::Connect,
                "delete" => Method::Delete,
                "get" => Method::Get,
                "head" => Method::Head,
                "options" => Method::Options,
                "patch" => Method::Patch,
                "post" => Method::Post,
                "put" => Method::Put,
                "trace" => Method::Trace,
                _ => return Err(io::Error::new(ErrorKind::InvalidData, "Invalid Request")),
            };
            target = Cow::Owned(vals[1].to_owned());
            version = match vals[2] {
                "HTTP/1.1" => (1, 1, 0),
                _ => return Err(io::Error::new(ErrorKind::InvalidData, "Invalid Request")),
            };
        }
        let mut headers: HashMap<Cow<'a, str>, Cow<'a, str>> = HashMap::new();
        for line in lines {
            let line = line.split_once(':');
            if let Some((key, value)) = line {
                headers.insert(
                    Cow::Owned(key.trim().to_owned()),
                    Cow::Owned(value.trim().to_owned()),
                );
            };
        }
        Ok(Self {
            method,
            target,
            version,
            headers,
            stream,
        })
    }
}
