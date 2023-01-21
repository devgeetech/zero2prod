use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to get configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port); 
    let listener = TcpListener::bind(address).expect("Failed to bind port");

    run(listener)?.await
}
