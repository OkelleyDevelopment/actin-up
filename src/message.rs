//! The enumeration of message types between actors.

use tokio::sync::oneshot;

// To model the supported operations in the 'server'. Originally just the type of operation,
// I've now elected to create this enum with state inside the variant so it's
// nearly impossible to send an incorrect message.
pub enum Message<K, V> {
    GET {
        key: K,
        resp: oneshot::Sender<Option<V>>,
    },
    INSERT {
        key: K,
        val: V,
        resp: oneshot::Sender<Option<V>>,
    },
    DELETE {
        key: K,
        resp: oneshot::Sender<Option<V>>,
    },
    QUERY {
        key: K,
        func: Box<dyn Send + 'static + FnOnce(&V)>,
    },
}
