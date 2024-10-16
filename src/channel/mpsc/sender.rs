#[cfg(not(feature = "crossbeam"))]
use std::sync::mpsc::Sender;

use crate::channel::generic_sender::GenericSender;

impl<T> GenericSender<T> for Sender<T>
where
    T: Send + 'static,
{
    fn send(&self, msg: T) -> Result<(), Box<dyn std::error::Error>> {
        self.send(msg).map_err(std::convert::Into::into)
    }
}
