extern crate reqwest;
extern crate simple_server;

use simple_server::{Server, Method, StatusCode};

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let host = "0.0.0.0";
    let port = env::var("PORT").unwrap_or(String::from("7878"));

    println!("Starting server on http://{}:{}", host, port);

    let server = Server::new(|request, mut response| {
        let method = request.method();
        let uri = request.uri();
        println!("Request received. {} {}", method, uri.path());

        // workaround for https://github.com/steveklabnik/simple-server/issues/88
        if let (&Method::GET, "/") = (method, uri.path()) {
            let mut f = File::open("public/index.html")?;

            let mut source = Vec::new();

            f.read_to_end(&mut source)?;

            return Ok(response.body(source)?);
        }

        if method == &Method::GET && uri.path().starts_with("/versions") {
            // /versions/ is 10 chars long
            let crate_name = &uri.path()[10..];

            // DONT DO THIS KIDS DEAR GOD
            let mut resp = reqwest::get(&format!("https://crates.io/api/v1/crates/{}/versions", crate_name)).unwrap();

            let mut content = Vec::new();
            resp.read_to_end(&mut content).unwrap();

            response.header("Content-Type", "application/json");
            return Ok(response.body(content)?);
        }
        

        response.status(StatusCode::NOT_FOUND);
        Ok(response.body("<h1>404</h1><p>Not found!<p>".as_bytes().to_vec())?)
    });

    server.listen(host, &port);
}
