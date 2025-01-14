use std::{
    borrow::Cow,
    collections::HashMap,
    io::{self, BufRead, BufReader, ErrorKind},
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
    pub fn get<T: AsRef<str>>(&self, value: &'a T) -> Option<&Cow<'a, str>> {
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
    fn try_from(value: TcpStream) -> Result<Self, Self::Error> {
        let mut rdr = BufReader::new(&value).lines();
        loop {
            let line = rdr.next();
            if line.is_none() {
                continue;
            }
            let line = line.unwrap()?;
            let vals: Vec<_> = line.split_whitespace().collect();
            if vals.len() != 3 {
                return Err(io::Error::new(ErrorKind::InvalidData, "Invalid Request"));
            }
            let method = match vals[0].to_lowercase().as_str() {
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
            let target: Cow<'_, str> = Cow::Owned(vals[1].to_owned());
            let version = match vals[2] {
                "HTTP/1.1" => (1, 1, 0),
                _ => return Err(io::Error::new(ErrorKind::InvalidData, "Invalid Request")),
            };
            return Ok(Self {
                method,
                target,
                version,
                headers: HashMap::default(),
                stream: value,
            });
        }
    }
}
