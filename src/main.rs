use crate::server::{ServiceBusServer, ServiceBusServerConfiguration};

mod routes;
mod store;
mod server;

fn main() {

    let cfg =
        ServiceBusServerConfiguration::default()
            .with_address("127.0.0.1:1469".to_string())
            .with_store_path("".to_string())
            .with_console_sink();

    ServiceBusServer::start(cfg);
}
