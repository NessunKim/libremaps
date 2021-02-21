extern crate libremaps;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    libremaps::run().await
}
