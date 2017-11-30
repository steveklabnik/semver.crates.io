extern crate simple_server;

use simple_server::{Server, Method, StatusCode};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let host = "127.0.0.1";
    let port = "7878";

    println!("Starting server on http://{}:{}", host, port);
    let server = Server::new(|request, mut response| {
        println!("Request received. {} {}", request.method(), request.uri());

        // workaround for https://github.com/steveklabnik/simple-server/issues/88
        if let (&Method::GET, "/") = (request.method(), request.uri().path()) {
            let mut f = File::open("public/index.html")?;

            let mut source = Vec::new();

            f.read_to_end(&mut source)?;

            return Ok(response.body(source)?);
        }

        response.status(StatusCode::NOT_FOUND);
        Ok(response.body("<h1>404</h1><p>Not found!<p>".as_bytes().to_vec())?)
    });

    server.listen(host, port);
}
