use std::{
    io::{self, BufReader, ErrorKind, Read, Write},
    str::FromStr,
};

use flate2::{write::ZlibEncoder, Compression};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    Gzip,
    None,
}

impl Default for Encoding {
    fn default() -> Self {
        Self::None
    }
}

impl FromStr for Encoding {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f = |v| match v {
            "gzip" => Ok(Self::Gzip),
            _ => Err(io::Error::new(ErrorKind::Unsupported, "Unknown Type")),
        };
        for i in s.trim().split(',').map(|v| v.trim()) {
            if let Ok(v) = f(i) {
                return Ok(v);
            }
        }
        Err(io::Error::new(ErrorKind::Unsupported, "Unknown Type"))
    }
}

impl Encoding {
    pub fn encode<R: io::Read, W: io::Write>(&self, w: &mut W, data: R) -> io::Result<()> {
        let mut buffer = [0; 1024];
        match self {
            Self::Gzip => {
                let mut e = ZlibEncoder::new(w, Compression::default());
                let mut reader = BufReader::new(data);
                loop {
                    let n = reader.read(&mut buffer)?;
                    if n == 0 {
                        break;
                    }
                    e.write_all(&buffer[0..n])?;
                }
                e.finish()?;
            }
            Self::None => {
                let mut reader = BufReader::new(data);
                loop {
                    let n = reader.read(&mut buffer)?;
                    if n == 0 {
                        break;
                    }
                    w.write_all(&buffer[0..n])?;
                }
                w.flush()?;
            }
        }
        Ok(())
    }
}
