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

#[derive(Clone)]
pub struct Router {
    routes: HashMap<Route, Arc<dyn Fn(Request) -> Response + Sync + Send>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn r#match(
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

    pub fn get(
        &mut self,
        path: &str,
        handler: impl Fn(Request) -> Response + Send + Sync + 'static,
    ) -> &mut Self {
        self.r#match(vec![Method::Get], path, handler)
    }

    pub fn post(
        &mut self,
        path: &str,
        handler: impl Fn(Request) -> Response + Send + Sync + 'static,
    ) -> &mut Self {
        self.r#match(vec![Method::Post], path, handler)
    }

    pub fn handle(&self, request: Request) -> Response {
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
