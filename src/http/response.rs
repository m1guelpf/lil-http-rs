use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use serde_json::Value;

use super::Method;

const CRLF: &str = "\r\n";

/// The HTTP status code of a response.
///
/// See [RFC 7231](https://tools.ietf.org/html/rfc7231#section-6) for more information.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StatusCode {
    /// 200 OK
    Ok,
    /// 404 Not Found
    NotFound,
    /// 400 Bad Request
    BadRequest,
    /// 405 Method Not Allowed
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

/// An HTTP 1.1 response.
///
/// See [RFC 2616](https://www.rfc-editor.org/rfc/rfc2616#section-6) for more information.
#[derive(Debug, Clone)]
pub struct Response {
    /// The status code of the response.
    pub status: StatusCode,
    /// The HTTP headers of the response.
    pub headers: HashMap<String, String>,
    /// The body of the response.
    pub body: Option<String>,
}

impl Response {
    /// Create a new text response with the given content.
    ///
    /// # Example
    /// ```
    /// use lil_http::{Response, StatusCode};
    ///
    /// let response = Response::text("Hello, World!");
    ///
    /// assert_eq!(response.status, StatusCode::Ok);
    /// assert_eq!(response.headers.get("Content-Type"), Some(&"text/plain".to_string()));
    /// assert_eq!(response.body, Some("Hello, World!".to_string()));
    /// ```
    #[allow(clippy::must_use_candidate)]
    pub fn text(body: &str) -> Self {
        Self {
            status: StatusCode::Ok,
            headers: vec![("Content-Type".into(), "text/plain".into())]
                .into_iter()
                .collect(),
            body: Some(body.to_string()),
        }
    }

    /// Create a new JSON response with the given content.
    ///
    /// # Example
    /// ```
    /// use lil_http::{Response, StatusCode};
    /// use serde_json::json;
    ///
    /// let response = Response::json(&json!({
    ///    "message": "Hello, World!"
    /// }));
    ///
    /// assert_eq!(response.status, StatusCode::Ok);
    /// assert_eq!(response.headers.get("Content-Type"), Some(&"application/json".to_string()));
    /// assert_eq!(response.body, Some("{\"message\":\"Hello, World!\"}".to_string()));
    /// ```
    #[allow(clippy::must_use_candidate)]
    pub fn json(body: &Value) -> Self {
        Self {
            status: StatusCode::Ok,
            headers: vec![("Content-Type".into(), "application/json".into())]
                .into_iter()
                .collect(),
            body: Some(body.to_string()),
        }
    }

    /// Create a 404 Not Found response.
    ///
    /// # Example
    /// ```
    /// use lil_http::{Response, StatusCode};
    ///
    /// let response = Response::not_found();
    ///
    /// assert_eq!(response.status, StatusCode::NotFound);
    /// assert_eq!(response.body, Some("Not Found".to_string()));
    /// ```
    #[allow(clippy::must_use_candidate)]
    pub fn not_found() -> Self {
        let mut resp = Self::text("Not Found");
        resp.status = StatusCode::NotFound;

        resp
    }

    /// Create a 400 Bad Request response.
    ///
    /// # Example
    /// ```
    /// use lil_http::{Response, StatusCode};
    ///
    /// let response = Response::invalid_request();
    ///
    /// assert_eq!(response.status, StatusCode::BadRequest);
    /// assert_eq!(response.body, Some("Invalid Request".to_string()));
    /// ```
    #[allow(clippy::must_use_candidate)]
    pub fn invalid_request() -> Self {
        let mut resp = Self::text("Invalid Request");
        resp.status = StatusCode::BadRequest;

        resp
    }

    /// Create a 405 Method Not Allowed response.
    /// The `methods` parameter is a list of allowed methods.
    /// The `Allow` header will be set to a comma-separated list of the methods.
    ///
    /// # Example
    /// ```
    /// use lil_http::{Response, StatusCode, Method};
    ///
    /// let response = Response::method_not_allowed(&[Method::Get, Method::Post]);
    ///
    /// assert_eq!(response.status, StatusCode::MethodNotAllowed);
    /// assert_eq!(response.headers.get("Allow"), Some(&"GET, POST".to_string()));
    /// assert_eq!(response.body, Some("Method Not Allowed".to_string()));
    /// ```
    #[allow(clippy::must_use_candidate)]
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

/// Convert a Response object into a HTTP 1.1 response string.
impl ToString for Response {
    fn to_string(&self) -> String {
        let mut str_response = String::new();

        str_response.push_str(&format!("HTTP/1.1 {}{CRLF}", self.status));
        for (name, value) in &self.headers {
            str_response.push_str(&format!("{name}: {value}{CRLF}"));
        }
        str_response.push_str(CRLF);
        str_response.push_str(self.body.as_deref().unwrap_or_default());

        str_response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_to_string() {
        let response = Response::text("Hello, World!");

        assert_eq!(
            response.to_string(),
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!"
        );
    }
}
