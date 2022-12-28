use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use serde_json::Value;

use super::Method;

const CRLF: &str = "\r\n";

#[derive(Debug, Clone)]
pub enum StatusCode {
    Ok,
    NotFound,
    BadRequest,
    MethodNotAllowed,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ok => "200 OK",
                Self::NotFound => "404 Not Found",
                Self::BadRequest => "400 Bad Request",
                Self::MethodNotAllowed => "405 Method Not Allowed",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn text(body: &str) -> Self {
        Self {
            status: StatusCode::Ok,
            headers: vec![("Content-Type".into(), "text/plain".into())]
                .into_iter()
                .collect(),
            body: body.to_string(),
        }
    }

    pub fn json(body: &Value) -> Self {
        Self {
            status: StatusCode::Ok,
            headers: vec![("Content-Type".into(), "application/json".into())]
                .into_iter()
                .collect(),
            body: body.to_string(),
        }
    }

    pub fn not_found() -> Self {
        let mut resp = Self::text("Not Found");
        resp.status = StatusCode::NotFound;

        resp
    }

    pub fn invalid_request() -> Self {
        let mut resp = Self::text("Invalid Request");
        resp.status = StatusCode::BadRequest;

        resp
    }

    pub fn method_not_allowed(methods: &[Method]) -> Self {
        let mut resp = Self::text("Method Not Allowed");

        resp.status = StatusCode::MethodNotAllowed;
        resp.headers.insert(
            "Allow".into(),
            methods
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .join(", "),
        );

        resp
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut str_response = String::new();

        str_response.push_str(&format!("HTTP/1.1 {}{CRLF}", self.status));
        for (name, value) in &self.headers {
            str_response.push_str(&format!("{name}: {value}{CRLF}"));
        }
        str_response.push_str(CRLF);
        str_response.push_str(&self.body);

        str_response
    }
}
