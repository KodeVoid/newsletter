mod handlers;
mod routes;
use newsletter::run;
// Alternative, more explicit version:
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let server = run(None)?; // Get the Server handle
    server.await // Run the server to completion
}
