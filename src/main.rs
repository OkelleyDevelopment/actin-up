use actor::{
    actor::{query, ActorServer},
    logging::await_response,
    message::Message,
};
use std::collections::HashMap;
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    oneshot,
};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut servers_to_start: Vec<ActorServer<&str, &str>> = Vec::new();
    let mut client_handles: HashMap<Uuid, UnboundedSender<Message<_, _>>> = HashMap::new();
    // Spawn the unique servers
    for _ in 0..=1 {
        // Creates the server and sender
        let (sender, actor_reciever) = mpsc::unbounded_channel();
        let server = ActorServer::new(actor_reciever);
        client_handles.insert(server.id(), sender);
        servers_to_start.push(server);
    }

    // Start the actor threads
    for s in servers_to_start {
        s.start();
    }

    println!("[Messenger]: Beginning tasks...");
    let fish = ["salmon", "bass"];
    for (index, key) in client_handles.keys().enumerate() {
        let handle = client_handles.get(key).unwrap();
        let (sender, _recv) = oneshot::channel();
        let _ = handle.send(Message::INSERT {
            key: "fish",
            val: fish[index],
            resp: sender,
        });

        let (sender, mut recv) = oneshot::channel();

        let _ = handle.send(Message::GET {
            key: "fish",
            resp: sender,
        });

        await_response(&mut recv, "Messenger").await;

        let handle = client_handles.get(key).unwrap().clone();
        let res = query(
            "fish",
            Box::new(move |v| if *v == "bass" { 42 } else { 25 }),
            handle,
        )
        .await;

        match res.handle.await {
            Ok(v) => println!("The result recieved for the query: {}", v),
            Err(e) => panic!("Failed to query. Message: {}", e),
        }
    }
    println!("Main is done...");
    std::process::exit(0);
}
