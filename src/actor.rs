use crate::operations::{Message, Operation};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver},
    task::JoinHandle,
};
use uuid::Uuid;
pub struct ActorServer<K, V>
where
    K: Eq + PartialEq + Clone + Hash + Send + Sync + 'static,
    V: Display + Clone + Send + Sync + 'static,
{
    /// Actors identifier
    id: Uuid,
    /// This actor servers' recieving channel
    reciever: mpsc::UnboundedReceiver<Message<K, V>>,
    /// Ideally a mechanism to help back-log messages recv'd
    message_queue: Vec<Message<K, V>>,
    /// This actor servers' local "db"
    db: HashMap<K, V>,
}

impl<K, V> ActorServer<K, V>
where
    K: Eq + PartialEq + Clone + Hash + Send + Sync + ToString + 'static,
    V: Display + Clone + Send + Sync + 'static,
{
    pub fn new(reciever: UnboundedReceiver<Message<K, V>>) -> Self {
        let id: Uuid = Uuid::new_v4();
        ActorServer {
            id,
            reciever,
            message_queue: Vec::new(),
            db: HashMap::new(),
        }
    }
    pub fn start(mut self) -> JoinHandle<()> {
        // Create our "server instance"
        //
        // TODO:
        // - Investigate other channel types / methods of handle recieving messages
        //   and handling conflicts

        tokio::spawn(async move {
            loop {
                match &self.reciever.try_recv() {
                    Ok(message) => {
                        self.message_queue.push(message.clone());
                    }
                    Err(_) => {}
                }

                if let Some(m) = &self.message_queue.pop() {
                    self.handle_message(m.clone());
                }
            }
        })
    }

    pub fn handle_message(&mut self, m: Message<K, V>) {
        match m.operation {
            Operation::GET { ref key } => {
                if let Some(val) = self.db.get(key) {
                    println!(
                        "[Actor Server {}]: Recieved a new message to get value for key: {}",
                        self.id,
                        key.to_string()
                    );
                    // TODO: Consider handling the error on responding.
                    let _ = m.res_channel.send(val.clone());
                }
            }
            Operation::INSERT { key, value } => {
                println!(
                    "[Actor Server {}]: Recieved a new message insert value: {}",
                    self.id, value
                );
                self.db.insert(key, value);
            }
            Operation::DELETE { ref key } => {
                if let Some(val) = self.db.remove(key) {
                    println!("[Actor Server {}]: Recieved a new message to delete and return the value: {}", self.id, val);
                    let _ = m.res_channel.send(val);
                }
            }
        }
    }
}
