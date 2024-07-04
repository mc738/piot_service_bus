use piot_http::server::configuration::ServerConfiguration;
use piot_http::server::Server;
use piot_log::{Log, LogConfiguration};
use crate::server::middleware::middleware;
use crate::server::routes::routes;

pub (crate) mod routes;
pub (crate) mod middleware;

pub struct ServiceBusServer {
    server_configuration: ServerConfiguration,
    log_configuration: LogConfiguration,
}


impl ServiceBusServer {

    pub fn create(address: String) -> ServiceBusServer {
        ServiceBusServer {
            server_configuration: ServerConfiguration {
                name: "PIoT Service Bus".to_string(),
                address,
                middleware: middleware(),
                routes: routes(),
            },
            log_configuration: LogConfiguration::default(),
        }
    }

    pub fn with_console_sink(mut self) -> ServiceBusServer {
        self.log_configuration.add_console_sink();
        self
    }

    pub fn start(self) {
        let server_cfg = self.server_configuration;
        let log_cfg = self.log_configuration;
        let server = Server::create(server_cfg);
        let log = Log::create(log_cfg).unwrap();

        server.start(log.get_logger());

        loop {}
    }

}