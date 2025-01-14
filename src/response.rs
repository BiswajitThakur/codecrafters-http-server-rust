use std::{
    borrow::Cow,
    collections::HashMap,
    io::{self, Write},
};

use crate::{Request, Status};

pub struct Response<'a, T: io::Read> {
    _version: (u8, u8, u8),
    _status: Status,
    _content_type: Cow<'a, str>,
    _content_length: usize,
    _headers: HashMap<Cow<'a, str>, Cow<'a, str>>,
    _body: Option<T>,
}

impl<'a, T: io::Read> Default for Response<'a, T> {
    fn default() -> Self {
        Self {
            _version: (1, 1, 0),
            _status: Status::NotFound,
            _content_type: "text/html".into(),
            _content_length: 0,
            _headers: HashMap::new(),
            _body: None,
        }
    }
}

impl<'a, T: io::Read> Response<'a, T> {
    pub fn version(self, version: (u8, u8, u8)) -> Self {
        Self {
            _version: version,
            ..self
        }
    }
    pub fn status(self, status: Status) -> Self {
        Self {
            _status: status,
            ..self
        }
    }
    pub fn content_type<U: Into<Cow<'a, str>>>(self, value: U) -> Self {
        Self {
            _content_type: value.into(),
            ..self
        }
    }
    pub fn content_length(self, length: usize) -> Self {
        Self {
            _content_length: length,
            ..self
        }
    }
    pub fn headers(self, header: HashMap<Cow<'a, str>, Cow<'a, str>>) -> Self {
        Self {
            _headers: header,
            ..self
        }
    }
    pub fn body(self, body: T) -> Self {
        Self {
            _body: Some(body),
            ..self
        }
    }
    pub fn send_to(&mut self, mut stream: Request<'_>) -> io::Result<()> {
        match self._version {
            (1, 1, 0) => {
                stream.write_all(b"HTTP/1.1 ")?;
                write!(stream, "{}", self._status)?;
                stream.write_all(b"\r\n")?;
            }
            _ => {}
        }
        write!(stream, "Content-Type: {}\r\n", self._content_type)?;
        write!(stream, "Content-Length: {}\r\n", self._content_length)?;
        stream.write_all(b"\r\n")?;
        let mut buffer = [0; 1024];
        if let Some(v) = self._body.as_mut() {
            loop {
                let v = v.read(&mut buffer)?;
                if v == 0 {
                    break;
                }
                stream.write_all(&buffer[0..v])?;
            }
        }
        Ok(())
    }
}
