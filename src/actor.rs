use crate::message::Message;
use std::fmt::Display;
use std::hash::Hash;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::Mutex;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver},
    task::JoinHandle,
};
use uuid::Uuid;

/// An agent in a network of >= 1 nodes.
///
/// This structure models a single actor with it's own backing
/// datastore.
pub struct ActorServer<K, V, T>
where
    K: Eq + PartialEq + Copy + Clone + Hash + Send + Sync + 'static,
    V: Display + Copy + Clone + Send + Sync + 'static,
    T: Send + Copy + Clone + 'static,
{
    /// Actors identifier
    id: Uuid,
    /// This actor servers' recieving channel
    reciever: mpsc::UnboundedReceiver<Message<K, V, T>>,
    /// Ideally a mechanism to help back-log messages recv'd
    // message_queue: Vec<Message<K, V, T>>,
    /// This actor servers' local "db"
    db: Arc<Mutex<HashMap<K, V>>>,
}

impl<K, V, T> ActorServer<K, V, T>
where
    K: Eq + PartialEq + Copy + Clone + Hash + Send + Sync + ToString + 'static,
    V: Display + Copy + Clone + Send + Sync + 'static,
    T: Send + Copy + Clone + ToString + 'static,
{
    /// A constructor method that will use a specified reciever
    /// and then default the `message_queue` and initialize the
    /// backing data store.
    pub fn new(reciever: UnboundedReceiver<Message<K, V, T>>) -> Self {
        let id: Uuid = Uuid::new_v4();
        ActorServer {
            id,
            reciever,
            // message_queue: Vec::new(),
            db: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Returns this nodes assigned id (`Uuid`)
    pub fn id(&self) -> Uuid {
        self.id
    }

    // TODO: This should write to a buffer that is protected
    //       to ensure we don't experience stdout resource starvation.
    pub fn log(&self, info: &str) {
        println!("[Server {}]: {}", self.id(), info);
    }

    /// Takes our actor and spins off a new tokio thread ("task").
    pub fn start(mut self) -> JoinHandle<()> {
        self.log(&format!("Starting up actor..."));
        tokio::spawn(async move {
            loop {
                match self.reciever.try_recv() {
                    Ok(m) => match m {
                        Message::GET { ref key, resp } => {
                            let db = self.db.lock().await;
                            println!(
                                "[Server {}]: Getting value from key: {}",
                                self.id(),
                                key.to_string()
                            );
                            let r = db.get(key).cloned();
                            let _ = resp.send(r);
                        }
                        Message::INSERT { key, value, resp } => {
                            let mut db = self.db.lock().await;
                            println!(
                                "[Server {}]: Inserting a new value: {}",
                                self.id(),
                                value.to_string()
                            );
                            db.insert(key.clone(), value.clone());
                            let _ = resp.send(());
                        }
                        Message::DELETE { ref key, resp } => {
                            let mut db = self.db.lock().await;
                            let r = db.remove(key);
                            let _ = resp.send(r);
                        }
                        Message::QUERY { key, func, resp } => {
                            let db = self.db.lock().await;
                            println!("[Server {}]: Arbitrary query ...", self.id());
                            if let Some(val) = db.get(&key) {
                                // perform arbitrary op and return result
                                let res = func(val.clone());
                                let _ = resp.send(Some(res.clone().to_string()));
                            }
                        }
                    },
                    Err(_e) => {} // TODO: determine what to do in this case
                }
            }
        })
    }
}

/// Utility function to generate a new actor server
pub fn generate_actor<K, V, T>() -> (ActorServer<K, V, T>, UnboundedSender<Message<K, V, T>>)
where
    K: Eq + PartialEq + Copy + Clone + Hash + Send + Sync + ToString + 'static,
    V: Display + Copy + Clone + Send + Sync + 'static,
    T: Send + Copy + Clone + 'static + ToString,
{
    // Future Considerations:
    // - Consider creating a bounded channel
    // - Perhaps wrap the `actor_reciever` as a type so this function becomes a method
    let (sender, actor_reciever) = mpsc::unbounded_channel();
    (ActorServer::new(actor_reciever), sender)
}
