use crate::message::Message;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;
use uuid::Uuid;

/// An agent in a network of >= 1 nodes.
///
/// This structure models a single actor with it's own backing
/// datastore.
pub struct ActorServer<K, V> {
    /// Actors identifier
    id: Uuid,
    /// This actor servers' recieving channel
    reciever: mpsc::UnboundedReceiver<Message<K, V>>,
    /// This actor servers' local "db"
    db: HashMap<K, V>,
}

impl<K, V> ActorServer<K, V> {
    /// A constructor method that will use a specified reciever
    /// and then default the `message_queue` and initialize the
    /// backing data store.
    pub fn new(reciever: UnboundedReceiver<Message<K, V>>) -> Self {
        let id: Uuid = Uuid::new_v4();
        ActorServer {
            id,
            reciever,
            db: HashMap::new(),
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
}

impl<K, V> ActorServer<K, V>
where
    K: Send + Eq + PartialEq + Hash + Clone + 'static,
    V: Display + Send + Clone + 'static,
{
    /// Takes our actor and spins off a new tokio thread ("task").
    pub fn start(mut self) {
        self.log("Starting up actor...");
        tokio::spawn(async move {
            loop {
                match self.reciever.recv().await.unwrap() {
                    Message::GET { key, resp } => drop(resp.send(self.db.get(&key).cloned())),
                    Message::INSERT { key, val, resp } => drop(resp.send(self.db.insert(key, val))),
                    Message::DELETE { key, resp } => drop(resp.send(self.db.remove(&key))),
                    Message::QUERY { key, func } => drop(self.db.get(&key).map(func)),
                }
            }
        });
    }
}

#[derive(Debug)]
pub struct ActorClient<U> {
    /// The channel the query response would come back on
    pub handle: oneshot::Receiver<U>,
}

impl<U> ActorClient<U> {
    /// Creates a new `ActorClient`
    pub fn new(handle: oneshot::Receiver<U>) -> Self {
        Self { handle }
    }
}

/// Send a query to an active node and perform some arbitary function, with any results
/// being sent back via interior channels.
pub async fn query<K, V, U>(
    key: K,
    func: Box<dyn 'static + Send + FnOnce(&V) -> U>,
    server: UnboundedSender<Message<K, V>>,
) -> ActorClient<U>
where
    V: Send + 'static,
    U: Send + 'static,
{
    let (send, recv) = oneshot::channel();
    drop(server.send(Message::QUERY {
        key,
        func: { Box::new(move |v| drop(send.send(func(v)))) },
    }));
    ActorClient { handle: recv }
}
