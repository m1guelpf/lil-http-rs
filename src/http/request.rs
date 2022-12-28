use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    io::Error,
};

use serde_json::Value;

const CRLF: &str = "\r\n";

/// The HTTP Method of a request.
///
/// See [RFC 7231](https://tools.ietf.org/html/rfc7231#section-4) for more information.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Method {
    /// HEAD method.
    Head,
    /// GET method.
    Get,
    /// POST method.
    Post,
    /// PUT method.
    Put,
    /// DELETE method.
    Delete,
}

impl From<String> for Method {
    fn from(val: String) -> Self {
        match val.as_str() {
            "HEAD" => Self::Head,
            "GET" => Self::Get,
            "POST" => Self::Post,
            "PUT" => Self::Put,
            "DELETE" => Self::Delete,
            _ => todo!(),
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Head => write!(f, "HEAD"),
            Self::Get => write!(f, "GET"),
            Self::Post => write!(f, "POST"),
            Self::Put => write!(f, "PUT"),
            Self::Delete => write!(f, "DELETE"),
        }
    }
}

/// The HTTP Body of a request.
///
/// See [RFC 7230](https://tools.ietf.org/html/rfc7230#section-3.3) for more information.
#[derive(Debug, Clone)]
pub enum Body {
    /// No body.
    None,
    /// A text/plain body.
    Text(String),
    /// A deserialized application/json body.
    Json(Value),
}

impl Body {
    /// # Panics
    ///
    /// Will panic if the content type is `application/json` and the body is not valid JSON.
    #[must_use]
    pub fn parse(body: String, content_type: Option<&String>) -> Self {
        match content_type {
            Some(content_type) => match content_type.as_str() {
                "application/json" => Self::Json(serde_json::from_str(&body).unwrap()),
                _ => Self::Text(body),
            },
            None => Self::Text(body),
        }
    }
}

impl Display for Body {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, ""),
            Self::Text(body) => write!(f, "{body}"),
            Self::Json(body) => write!(f, "{body}"),
        }
    }
}

/// An HTTP 1.1 request.
///
/// See [RFC 7230](https://tools.ietf.org/html/rfc7230) for more information.
#[derive(Debug, Clone)]
pub struct Request {
    /// The HTTP method of the request.
    pub method: Method,
    /// The path of the request.
    pub path: String,
    /// The parsed query of the request.
    pub query: HashMap<String, String>,
    /// The parsed headers of the request.
    pub headers: HashMap<String, String>,
    /// The body of the request.
    pub body: Body,
}

/// Try to parse a request object from a buffer.
///
/// # Errors
///
/// Will return an error if the buffer is empty.
///
/// # Panics
///
/// Will panic if the request method is not valid.
impl TryFrom<&[u8; 1024]> for Request {
    type Error = Error;

    fn try_from(buf: &[u8; 1024]) -> Result<Self, Self::Error> {
        let mut body = Body::None;
        let mut headers = HashMap::new();

        if buf[0] == 0 {
            return Err(Error::new(std::io::ErrorKind::InvalidData, "Empty request"));
        }

        let mut buff_read: usize = 2;
        let mut lines = buf.split(|&byte| byte == b'\n');

        let request_line = lines.next().unwrap();
        buff_read += request_line.len() + 1;
        let mut request_line = request_line.split(|&byte| byte == b' ');

        let method: Method = String::from_utf8_lossy(request_line.next().unwrap())
            .to_string()
            .into();

        let uri = String::from_utf8_lossy(request_line.next().unwrap());
        let mut uri = uri.splitn(2, |byte| byte == '?');

        let path = uri.next().unwrap().trim().to_string();

        let query = uri.next().map_or_else(HashMap::new, |query| {
            query
                .trim()
                .split('&')
                .map(|pair| {
                    let mut pair = pair.split('=');
                    let key = pair.next().unwrap().trim().to_string();
                    let value = pair.next().unwrap().trim().to_string();

                    (key, value)
                })
                .collect::<HashMap<String, String>>()
        });

        for line in lines {
            if line == b"\r" {
                break;
            }

            let mut header = line.splitn(2, |&byte| byte == b':');
            let name = header.next().unwrap();
            let value = header.next().unwrap();

            let value = String::from_utf8_lossy(value).trim().to_string();
            let name = String::from_utf8_lossy(name).trim().to_string();

            headers.insert(name, value);
            buff_read += line.len() + 1;
        }

        if let Some(content_length) = headers.get("Content-Length") {
            let content_length = content_length.parse::<usize>().unwrap();

            body = Body::parse(
                String::from_utf8_lossy(&buf[buff_read..buff_read + content_length])
                    .trim()
                    .to_string(),
                headers.get("Content-Type"),
            );
        }

        Ok(Self {
            method,
            path,
            query,
            headers,
            body,
        })
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str_request = String::new();

        str_request.push_str(&format!("{} {} HTTP/1.1{CRLF}", self.method, self.path));
        for (name, value) in &self.headers {
            str_request.push_str(&format!("{name}: {value}{CRLF}"));
        }
        str_request.push_str(CRLF);
        str_request.push_str(&self.body.to_string());

        write!(f, "{str_request}")
    }
}
