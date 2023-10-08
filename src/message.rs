//! The enumeration of message types between actors.

use tokio::sync::oneshot;

// To model the supported operations in the 'server'. Originally just the type of operation,
// I've now elected to create this enum with state inside the variant so it's
// nearly impossible to send an incorrect message.
pub enum Message<K, V, T>
where
    K: Copy + Clone + Send + Sync,
    V: Copy + Clone + Send + Sync,
    T: Copy + Clone + Send,
{
    GET {
        key: K,
        resp: oneshot::Sender<Option<V>>,
    },
    INSERT {
        key: K,
        value: V,
        resp: oneshot::Sender<()>,
    },
    DELETE {
        key: K,
        resp: oneshot::Sender<Option<V>>,
    },
    QUERY {
        key: K,
        func: Box<dyn FnOnce(V) -> T + Send>,
        resp: oneshot::Sender<Option<String>>,
    },
}

#[derive(Clone, Copy)]
pub enum QueryResponse {
    Count(usize),
}

impl ToString for QueryResponse {
    fn to_string(&self) -> String {
        match self {
            QueryResponse::Count(c) => c.to_string(),
        }
    }
}
