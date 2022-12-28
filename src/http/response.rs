use crate::{Body, Method};
use serde_json::Value;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

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
    pub status_code: StatusCode,
    /// The HTTP headers of the response.
    pub headers: HashMap<String, String>,
    /// The body of the response.
    pub body: Body,
}

impl Response {
    /// Create an empty response with the 200 OK status code.
    ///
    /// # Example
    /// ```
    /// use lil_http::Response;
    /// # use lil_http::{StatusCode, Body};
    ///
    /// let response = Response::ok();
    ///
    /// # assert_eq!(response.body, Body::None);
    /// # assert_eq!(response.headers.len(), 0);
    /// # assert_eq!(response.status, StatusCode::Ok);
    /// ```
    #[allow(clippy::must_use_candidate)]
    pub fn ok() -> Self {
        Self {
            body: Body::None,
            status_code: StatusCode::Ok,
            headers: HashMap::new(),
        }
    }

    /// Create a new text response with the given content.
    ///
    /// # Example
    /// ```
    /// use lil_http::Response;
    /// # use lil_http::{StatusCode, Body};
    ///
    /// let response = Response::text("Hello, World!");
    ///
    /// # assert_eq!(response.status, StatusCode::Ok);
    /// # assert_eq!(response.body, Body::Text("Hello, World!".to_string()));
    /// # assert_eq!(response.headers.get("Content-Type"), Some(&"text/plain".to_string()));
    /// ```
    #[allow(clippy::must_use_candidate)]
    pub fn text(body: &str) -> Self {
        Self::ok()
            .header("Content-Type", "text/plain")
            .body(Body::Text(body.to_string()))
    }

    /// Create a new JSON response with the given content.
    ///
    /// # Example
    /// ```
    /// use lil_http::Response;
    /// # use lil_http::{StatusCode, Body};
    /// use serde_json::json;
    ///
    /// let response = Response::json(&json!({
    ///    "message": "Hello, World!"
    /// }));
    ///
    /// # assert_eq!(response.status, StatusCode::Ok);
    /// # assert_eq!(response.body, Body::Json(json!({ "message": "Hello, World!" })));
    /// # assert_eq!(response.headers.get("Content-Type"), Some(&"application/json".to_string()));
    /// ```
    #[allow(clippy::must_use_candidate)]
    pub fn json(body: &Value) -> Self {
        Self::ok()
            .header("Content-Type", "application/json")
            .body(Body::Json(body.to_owned()))
    }

    /// Create a 404 Not Found response.
    ///
    /// # Example
    /// ```
    /// use lil_http::Response;
    /// # use lil_http::{StatusCode, Body};
    ///
    /// let response = Response::not_found();
    ///
    /// # assert_eq!(response.status, StatusCode::NotFound);
    /// # assert_eq!(response.body, Body::Text("Not Found".to_string()));
    /// # assert_eq!(response.headers.get("Content-Type"), Some(&"text/plain".to_string()));
    /// ```
    #[allow(clippy::must_use_candidate)]
    pub fn not_found() -> Self {
        Self::text("Not Found").status(StatusCode::NotFound)
    }

    /// Create a 400 Bad Request response.
    ///
    /// # Example
    /// ```
    /// use lil_http::Response;
    /// # use lil_http::{StatusCode, Body};
    ///
    /// let response = Response::invalid_request();
    ///
    /// # assert_eq!(response.status, StatusCode::BadRequest);
    /// # assert_eq!(response.body, Body::Text("Invalid Request".to_string()));
    /// # assert_eq!(response.headers.get("Content-Type"), Some(&"text/plain".to_string()));
    /// ```
    #[allow(clippy::must_use_candidate)]
    pub fn invalid_request() -> Self {
        Self::text("Invalid Request").status(StatusCode::BadRequest)
    }

    /// Create a 405 Method Not Allowed response.
    /// The `methods` parameter is a list of allowed methods.
    /// The `Allow` header will be set to a comma-separated list of the methods.
    ///
    /// # Example
    /// ```
    /// use lil_http::{Response, Method};
    /// # use lil_http::{StatusCode, Body};
    ///
    /// let response = Response::method_not_allowed(&[Method::Get, Method::Post]);
    ///
    /// # assert_eq!(response.status, StatusCode::MethodNotAllowed);
    /// # assert_eq!(response.body, Body::Text("Method Not Allowed".to_string()));
    /// # assert_eq!(response.headers.get("Allow"), Some(&"GET, POST".to_string()));
    /// ```
    #[allow(clippy::must_use_candidate)]
    pub fn method_not_allowed(methods: &[Method]) -> Self {
        let mut methods = methods
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<String>>();
        methods.sort();

        Self::text("Method Not Allowed")
            .status(StatusCode::MethodNotAllowed)
            .header("Allow", &methods.join(", "))
    }

    /// Set the status code of the response.
    ///
    /// # Example
    ///
    /// ```
    /// use lil_http::{Response, StatusCode};
    ///
    /// let response = Response::text("Nope").status(StatusCode::NotFound);
    ///
    /// # assert_eq!(response.status, StatusCode::NotFound);
    pub fn status(&mut self, code: StatusCode) -> Self {
        self.status_code = code;

        self.to_owned()
    }

    /// Add a header to the response.
    /// If the header already exists, it will be overwritten.
    ///
    /// # Example
    /// ```
    /// use lil_http::Response;
    ///
    /// let response = Response::text("Hello, World!")
    ///    .header("X-Example", "test-header");
    ///
    /// # assert_eq!(response.headers.get("X-Example"), Some(&"test-header".to_string()));
    pub fn header(&mut self, name: &str, value: &str) -> Self {
        self.headers.insert(name.into(), value.into());

        self.to_owned()
    }

    /// Set the body of the response.
    ///
    /// # Example
    /// ```
    /// use lil_http::{Response, Body};
    /// use serde_json::json;
    ///
    /// let response = Response::ok()
    ///    .body(Body::Json(json!({ "message": "Hello, World!" })));
    ///
    /// # assert_eq!(response.body, Body::Json(json!({ "message": "Hello, World!" })));
    pub fn body(&mut self, body: Body) -> Self {
        self.body = body;

        self.to_owned()
    }
}

/// Convert a Response object into a HTTP 1.1 response string.
impl ToString for Response {
    fn to_string(&self) -> String {
        let mut str_response = String::new();

        str_response.push_str(&format!("HTTP/1.1 {}{CRLF}", self.status_code));
        for (name, value) in &self.headers {
            str_response.push_str(&format!("{name}: {value}{CRLF}"));
        }
        str_response.push_str(CRLF);
        str_response.push_str(&self.body.to_string());

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
