use cookie::Cookie;
use http::header::HeaderValue;
use tide::Response;
use tide::middleware::{Cors, Origin};

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {

    let mut portfolios = tide::new();
    portfolios.at("/portfolios").get(|_| async { Response::new(200) });

    let mut v1 = tide::new();
    v1.at("/v1").nest(portfolios);

    let mut api = tide::new();
    api.at("/api").nest(v1);


    println!("Server Listening ...");
    api.listen("127.0.0.1:8080").await?;
    Ok(())
}
