use std::fmt::Display;
use std::hash::Hash;
use tokio::sync::mpsc::UnboundedSender;

// To model the supported operations in the 'server'
#[derive(Debug, Clone)]
pub enum Operation<K, V> {
    GET { key: K },
    INSERT { key: K, value: V },
    DELETE { key: K },
}

#[derive(Clone)]
pub struct Message<K, V>
where
    K: Eq + PartialEq + Clone + Hash + Send + Sync + 'static,
    V: Display + Clone + Send + Sync + 'static,
{
    pub operation: Operation<K, V>,
    pub res_channel: UnboundedSender<V>,
}

impl<K, V> Message<K, V>
where
    K: Eq + PartialEq + Clone + Hash + Send + Sync + 'static,
    V: Display + Clone + Send + Sync + 'static,
{
    pub fn new(operation: Operation<K, V>, res_channel: UnboundedSender<V>) -> Self {
        Self {
            operation,
            res_channel,
        }
    }
}
