use std::process::Command;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use piot_log::Logger;
use rusqlite::Connection;
use crate::store::v1::init::initialize;

pub (crate) struct StoreAgent {
    handler: JoinHandle<()>,
    //logger: Logger,
    //connection: rusqlite::Connection,
    store: Store
}

#[derive(Clone)]
pub (crate) struct Store {
    sender: Sender<StoreCommand>

}

pub enum StoreCommand {
    Status(mpsc::Sender<StoreStatus>),
    CreateTopic(CreateTopicRequest, Sender<CreateTopicResponse>)
}

pub struct StoreStatus {}

pub struct CreateTopicRequest {
    id: Option<String>,
    name: String,
}

pub enum CreateTopicResponse {
    Success(CreateTopicSuccessResponse),
    AlreadyExists,
    Failure,
}

pub struct CreateTopicSuccessResponse {
    id: String,
    name: String,
}

impl StoreAgent {

    pub fn start(logger: Logger, store_path: String) -> StoreAgent {
        let (sender, receiver): (Sender<StoreCommand>, Receiver<StoreCommand>) = mpsc::channel();
        let store = Store { sender };
        let mut conn = rusqlite::Connection::open(store_path).unwrap();
        initialize(&mut conn).unwrap();

        logger.log_info("store-agent".to_string(), "Starting Store Agent".to_string()).unwrap();

        let handler = thread::spawn(move || loop {
            let item = receiver.recv().unwrap();
            hande_command(&conn, &logger, item);
        });

        StoreAgent {
            handler,
            store,
        }
    }
}


fn hande_command(conn: &Connection, logger: &Logger, command: StoreCommand) {

}