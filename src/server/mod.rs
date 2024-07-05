use piot_http::server::configuration::ServerConfiguration;
use piot_http::server::Server;
use piot_log::{Log, LogConfiguration};
use crate::server::middleware::middleware;
use crate::server::routes::routes;
use crate::store;

pub(crate) mod routes;
pub(crate) mod middleware;

pub struct ServiceBusServer {
    configuration: ServiceBusServerConfiguration,
}

pub struct ServiceBusServerConfiguration {
    store_path: String,
    server_configuration: ServerConfiguration,
    log_configuration: LogConfiguration,
}

impl ServiceBusServer {

    pub fn start(configuration: ServiceBusServerConfiguration) {
        let server_cfg = configuration.server_configuration;
        let log_cfg = configuration.log_configuration;
        let log = Log::create(log_cfg).unwrap();

        let logger = log.get_logger();

        logger.log_info("main".to_string(), "Starting PIoT Service Bus server".to_string()).unwrap();
        let server = Server::create(server_cfg);
        logger.log_success("main".to_string(), "PIoT Service Bus server started successful".to_string()).unwrap();


        logger.log_info("main".to_string(), "Starting PIoT Service Bus agent (v1)".to_string()).unwrap();
        let agent = store::v1::agent::StoreAgent::start(log.get_logger(), configuration.store_path);
        logger.log_success("main".to_string(), "PIoT Service Bus agent (v1) started successful".to_string()).unwrap();

        server.start(log.get_logger());

        loop {}
    }
}

impl ServiceBusServerConfiguration {
    pub fn create(address: String) -> ServiceBusServerConfiguration {
        ServiceBusServerConfiguration {
            store_path: "".to_string(),
            server_configuration: ServerConfiguration {
                name: "PIoT Service Bus".to_string(),
                address,
                middleware: middleware(),
                routes: routes(),
            },
            log_configuration: LogConfiguration::default(),
        }
    }

    pub fn default() -> ServiceBusServerConfiguration {
        ServiceBusServerConfiguration {
            store_path: "".to_string(),
            server_configuration: ServerConfiguration {
                name: "PIoT Service Bus".to_string(),
                address: "".to_string(),
                middleware: middleware(),
                routes: routes(),
            },
            log_configuration: LogConfiguration::default(),
        }
    }

    pub fn with_address(mut self, address: String) -> ServiceBusServerConfiguration {
        self.server_configuration.address = address;
        self
    }


    pub fn with_store_path(mut self, store_path: String) -> ServiceBusServerConfiguration {
        self.store_path = store_path;
        self
    }

    pub fn with_server_configuration(mut self, server_configuration: ServerConfiguration) -> ServiceBusServerConfiguration {
        self.server_configuration = server_configuration;
        self
    }

    pub fn with_log_configuration(mut self, log_configuration: LogConfiguration) -> ServiceBusServerConfiguration {
        self.log_configuration = log_configuration;
        self
    }

    pub fn with_console_sink(mut self) -> ServiceBusServerConfiguration {
        self.log_configuration.add_console_sink();
        self
    }
}