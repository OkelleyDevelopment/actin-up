//! Logging Utilities

use tokio::sync::oneshot::Receiver;

/// Utility function to await and log a response
pub async fn await_response<T>(listener: &mut Receiver<Option<T>>, name: &str) -> Option<T>
where
    T: ToString + Send + Clone,
{
    loop {
        match listener.try_recv() {
            Ok(val) => {
                // TODO: This should be more generic or moved outside this function
                println!(
                    "[{}]: Recieved: {}",
                    name,
                    val.clone().expect("a count should be present").to_string()
                );
                return val.clone();
            }
            Err(_e) => {}
        }
    }
}
