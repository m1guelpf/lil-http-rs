use std::{collections::HashMap, sync::Arc};

use crate::http::{Method, Request, Response};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Route {
    path: String,
    methods: Vec<Method>,
}

impl From<&Request> for Route {
    fn from(val: &Request) -> Self {
        Self {
            path: val.path.clone(),
            methods: vec![val.method],
        }
    }
}

/// The router is responsible for matching requests to handlers.
#[derive(Clone)]
pub struct Router {
    routes: HashMap<Route, Arc<dyn Fn(Request) -> Response + Sync + Send>>,
}

impl Router {
    /// Create a new router instance.
    pub(crate) fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    /// Match a route to a handler.
    pub(crate) fn r#match(
        &mut self,
        methods: Vec<Method>,
        path: &str,
        handler: impl Fn(Request) -> Response + Send + Sync + 'static,
    ) -> &mut Self {
        self.routes.insert(
            Route {
                methods,
                path: path.to_string(),
            },
            Arc::new(handler),
        );

        self
    }

    /// Register a GET route.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use lil_http::{Request, Response, Server};
    ///
    /// let mut server = Server::new().unwrap();
    ///
    /// server.routes.get("/", |request| {
    ///     println!("Received {} request to {}", request.method, request.path);
    ///     Response::text("Hello, World!")
    /// });
    /// ```
    pub fn get(
        &mut self,
        path: &str,
        handler: impl Fn(Request) -> Response + Send + Sync + 'static,
    ) -> &mut Self {
        self.r#match(vec![Method::Get], path, handler)
    }

    /// Register a POST route.
    ///
    /// # Example
    /// ```rust,no_run
    /// use lil_http::{Request, Response, Server};
    ///
    /// let mut server = Server::new().unwrap();
    ///
    /// server.routes.post("/", |request| {
    ///    println!("Received {} request to {}", request.method, request.path);
    ///    Response::text("Hello, World!")
    /// });
    /// ```
    pub fn post(
        &mut self,
        path: &str,
        handler: impl Fn(Request) -> Response + Send + Sync + 'static,
    ) -> &mut Self {
        self.r#match(vec![Method::Post], path, handler)
    }

    /// Register a PUT route.
    ///
    /// # Example
    /// ```rust,no_run
    /// use lil_http::{Request, Response, Server};
    ///
    /// let mut server = Server::new().unwrap();
    ///
    /// server.routes.put("/", |request| {
    ///    println!("Received {} request to {}", request.method, request.path);
    ///    Response::text("Hello, World!")
    /// });
    /// ```
    pub fn put(
        &mut self,
        path: &str,
        handler: impl Fn(Request) -> Response + Send + Sync + 'static,
    ) -> &mut Self {
        self.r#match(vec![Method::Put], path, handler)
    }

    /// Register a DELETE route.
    ///
    /// # Example
    /// ```rust,no_run
    /// use lil_http::{Request, Response, Server};
    ///
    /// let mut server = Server::new().unwrap();
    ///
    /// server.routes.delete("/", |request| {
    ///   println!("Received {} request to {}", request.method, request.path);
    ///    Response::text("Hello, World!")
    /// });
    /// ```
    pub fn delete(
        &mut self,
        path: &str,
        handler: impl Fn(Request) -> Response + Send + Sync + 'static,
    ) -> &mut Self {
        self.r#match(vec![Method::Delete], path, handler)
    }

    /// Handle an incoming request.
    /// If no route is found, a 404 response is returned.
    /// If a route is found, but the method is not allowed, a 405 response is returned.
    pub(crate) fn handle(&self, request: Request) -> Response {
        let path_routes = self
            .routes
            .iter()
            .filter(|(route, _)| route.path == request.path);

        if path_routes.clone().count() == 0 {
            return Response::not_found();
        }

        match path_routes
            .clone()
            .find(|(route, _)| route.methods.contains(&request.method))
        {
            Some((_, handler)) => handler(request),
            None => Response::method_not_allowed(
                &path_routes
                    .flat_map(|(route, _)| route.methods.clone())
                    .collect::<Vec<Method>>(),
            ),
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Body, StatusCode};

    #[test]
    fn test_router_default() {
        let router = Router::default();

        assert_eq!(router.routes.len(), 0);
    }

    #[test]
    fn test_route_from_request() {
        let request = Request {
            method: Method::Put,
            path: "/test/path".to_string(),
            query: HashMap::new(),
            headers: HashMap::new(),
            body: Body::None,
        };

        let route = Route::from(&request);

        assert_eq!(route.path, "/test/path");
        assert_eq!(route.methods, vec![Method::Put]);
    }

    #[test]
    fn test_router_match() {
        let mut router = Router::new();

        router.r#match(vec![Method::Put], "/test/path", |_request| {
            Response::text("")
        });

        assert_eq!(router.routes.len(), 1);
        assert!(router.routes.contains_key(&Route {
            path: "/test/path".to_string(),
            methods: vec![Method::Put]
        }));
    }

    #[test]
    fn test_router_get() {
        let mut router = Router::new();

        router.get("/test/path", |_request| Response::text(""));

        assert_eq!(router.routes.len(), 1);
        assert!(router.routes.contains_key(&Route {
            path: "/test/path".to_string(),
            methods: vec![Method::Get]
        }));
    }

    #[test]
    fn test_router_post() {
        let mut router = Router::new();

        router.post("/test/path", |_request| Response::text(""));

        assert_eq!(router.routes.len(), 1);
        assert!(router.routes.contains_key(&Route {
            path: "/test/path".to_string(),
            methods: vec![Method::Post]
        }));
    }

    #[test]
    fn test_router_handle() {
        let mut router = Router::new();

        router.get("/test/path", |_request| Response::text("test response"));

        let request = Request {
            method: Method::Get,
            path: "/test/path".to_string(),
            query: HashMap::new(),
            headers: HashMap::new(),
            body: Body::None,
        };

        let response = router.handle(request);

        assert_eq!(response.status_code, StatusCode::Ok);
        assert_eq!(response.body, Body::Text("test response".to_string()));
    }

    #[test]
    fn test_returns_404_if_no_routes_match() {
        let router = Router::new();

        let request = Request {
            method: Method::Get,
            path: "/".to_string(),
            query: HashMap::new(),
            headers: HashMap::new(),
            body: Body::None,
        };

        let response = router.handle(request);

        assert_eq!(response.status_code, StatusCode::NotFound);
        assert_eq!(response.body, Body::Text("Not Found".to_string()));
    }

    #[test]
    fn test_returns_405_if_method_does_not_match() {
        let mut router = Router::new();

        router
            .put("/test/path", |_| Response::text("test response"))
            .get("/test/path", |_| Response::text("test response"));

        let request = Request {
            method: Method::Post,
            path: "/test/path".to_string(),
            query: HashMap::new(),
            headers: HashMap::new(),
            body: Body::None,
        };

        let response = router.handle(request);

        assert_eq!(response.status_code, StatusCode::MethodNotAllowed);
        assert_eq!(response.body, Body::Text("Method Not Allowed".to_string()));
        assert_eq!(
            response.headers.get("Allow").unwrap(),
            &"GET, PUT".to_string()
        );
    }
}
