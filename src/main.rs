/*

Create a tokio task that contains a hashmap (can choose what data it maps). This will be the "server".

The server recieves messages via a tokio MPSC channel. This message communicates what operation is to be perfomred on the hashmap. For now the operations are:
- Insert key-val pair
- Return a clone of a value using a key
- Remove a value and return it

*Bonus*: Add a fourth operation that allows the caller to perform an arbitrary query on a value in the map and return the result of that query. In otherwords,
        give the server something like a `F: FnOnce(&Value) -> T` (or almost any T) and return `T` to the caller.

*/

use actor::{actor::ActorServer, message::Message};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (client_sender, actor_reciever) = mpsc::unbounded_channel();

    let (message_sender, mut client_reciever) = mpsc::unbounded_channel();

    let server = ActorServer::new(actor_reciever);
    server.start();

}
