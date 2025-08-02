use std::net::TcpListener;
mod routes;
mod handlers;
use actix_web::{ App, HttpServer };
use crate::routes::health;

// Modified to return Server handle instead of running to completion
pub fn run(listener: Option<TcpListener>) -> Result<actix_web::dev::Server, std::io::Error> {
    let server = HttpServer::new(|| { App::new().configure(health::init) });

    let server = match listener {
        Some(listener) => server.listen(listener)?,
        None => server.bind("127.0.0.1:3000")?,
    };

    Ok(server.run()) // Return the Server handle
}
