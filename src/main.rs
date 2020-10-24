mod models;
mod routes;
mod handlers;


#[tokio::main]
async fn main() {
    //init 
    let config: models::config::Config = toml::from_str(&std::fs::read_to_string(std::env::var("CONFIG_FILE")
            .unwrap_or("config.toml".to_string()))
        .expect("failed to open the toml file"))
        .expect("failed to parse the toml file");
    let routes = routes::embeds::embeds();

    //run
    println!("Starting the api at port {}", &config.server.port);
    warp::serve(routes).run((config.server.ip, config.server.port)).await;
}
