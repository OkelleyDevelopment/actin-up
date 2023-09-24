/*

Create a tokio task that contains a hashmap (can choose what data it maps). This will be the "server".

The server recieves messages via a tokio MPSC channel. This message communicates what operation is to be perfomred on the hashmap. For now the operations are:
- Insert key-val pair
- Return a clone of a value using a key
- Remove a value and return it

*Bonus*: Add a fourth operation that allows the caller to perform an arbitrary query on a value in the map and return the result of that query. In otherwords,
        give the server something like a `F: FnOnce(&Value) -> T` (or almost any T) and return `T` to the caller.

*/

/*

Limitations
- Currently have no way to support multi threaded simulations where
  a single actor gets a response from two other actors.

*/

use actor::{actor::ActorServer, operations::Message};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (client_sender, actor_reciever) = mpsc::unbounded_channel();

    let (message_sender, mut client_reciever) = mpsc::unbounded_channel();

    let server = ActorServer::new(actor_reciever);
    server.start();

    let message = Message::new(
        actor::operations::Operation::INSERT {
            key: "key",
            value: "some value",
        },
        message_sender.clone(),
    );

    // Set a key and value
    client_sender.send(message).expect("failed to send");

    let message_two = Message::new(
        actor::operations::Operation::GET { key: "key" },
        message_sender.clone(),
    );

    client_sender
        .send(message_two)
        .expect("failed to send message two");

    while let Some(m) = client_reciever.recv().await {
        println!("[Client]: Got response: {}", m);
        break;
    }

    let message_three = Message::new(
        actor::operations::Operation::DELETE { key: "key" },
        message_sender.clone(),
    );

    client_sender
        .send(message_three)
        .expect("failed to send message three");
    while let Some(m) = client_reciever.recv().await {
        println!("[Client]: Sent delete request, got response: <{}>", m);
        break;
    }

    println!("[Client]: jobs are done, shutting down...");
    std::process::exit(0);
}
