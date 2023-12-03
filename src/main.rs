mod config;
mod routes;
mod handlers;
mod domain;

use config::Config;

#[tokio::main]
async fn main() {
    let routes = routes::routes();

    // Create config structure
    let config: Config = Config::builder()
        .port(3030)
        .addr("0.0.0.0".to_string())
        .build();

    println!("Running example server on localhost:3030");
    warp::serve(routes).run((config.addr, config.port)).await;
}
