use actor::actor::generate_actor;
use actor::logging::await_response;
use actor::message::QueryResponse;
use actor::{actor::ActorServer, message::Message};
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot;
use uuid::Uuid;

// TODO: Createa formal actor for this...
/// Actor responsible for sending and receiving messages.
async fn message_handling_actor(
    client_handles: HashMap<Uuid, UnboundedSender<Message<&str, &str, QueryResponse>>>,
) {
    println!("[Messenger]: Beginning tasks...");
    let fish = ["salmon", "bass"];
    let mut index: usize = 0;
    for key in client_handles.keys() {
        let handle = client_handles.get(key).unwrap();
        let (sender, _recv) = oneshot::channel();
        let _ = handle.send(Message::INSERT {
            key: "fish",
            value: fish[index],
            resp: sender,
        });
        index = index + 1;

        let (sender, mut recv) = oneshot::channel();

        let _ = handle.send(Message::GET {
            key: "fish",
            resp: sender,
        });

        await_response(&mut recv, "Messenger").await;

        let (sender, mut recv) = oneshot::channel();
        let _ = handle.send(Message::QUERY {
            key: "fish",
            func: Box::new(|s| QueryResponse::Count(s.len())),
            resp: sender,
        });

        await_response(&mut recv, "Messenger").await;
    }
    println!("[Messenger]: is done...");
}

#[tokio::main]
async fn main() {
    let mut servers_to_start: Vec<ActorServer<&str, &str, QueryResponse>> = Vec::new();
    let mut client_handles: HashMap<Uuid, UnboundedSender<Message<_, _, _>>> = HashMap::new();
    // Spawn the unique servers
    for _ in 0..=1 {
        // Creates the server and sender
        let (server, client_sender) = generate_actor();
        client_handles.insert(server.id(), client_sender);
        servers_to_start.push(server);
    }

    // Start the actor threads
    for s in servers_to_start {
        s.start();
    }

    // Start the message handling actor
    let message_handle = tokio::spawn(message_handling_actor(client_handles.clone()));

    // Wait for the message handling actor to finish
    message_handle.await.expect("Message handling actor failed");

    println!("Main is done...");
    std::process::exit(0);
}
