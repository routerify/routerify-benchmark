use hyper::{Body, Request, Response, Server};
// Import the routerify prelude traits.
use routerify::prelude::*;
use routerify::{Router, RouterService};
use std::{convert::Infallible, net::SocketAddr};

// A handler for "/" page.
async fn home_handler(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Home page")))
}

// A handler for "/hello/:name" page.
async fn hello_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let name = req.param("name").unwrap();
    Ok(Response::new(Body::from(format!("Hey {}", name))))
}

fn router() -> Router<Body, Infallible> {
    Router::builder()
        .get("/", home_handler)
        .get("/hello/:name", hello_handler)
        .build()
        .unwrap()
}

#[tokio::main]
async fn main() {
    let router = router();

    // Create a Service from the router above to handle incoming requests.
    let service = RouterService::new(router).unwrap();

    // The address on which the server will be listening.
    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));

    // Create a server by passing the created service to `.serve` method.
    let server = Server::bind(&addr).serve(service);

    println!("App is running on: {}", addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}
