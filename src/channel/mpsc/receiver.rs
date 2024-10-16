#[cfg(not(feature = "crossbeam"))]
use std::sync::mpsc::Receiver;

use crate::channel::generic_receiver::GenericReceiver;

impl<T> GenericReceiver<T> for Receiver<T>
where
    T: Send + 'static,
{
    fn recv(&self) -> Result<T, Box<dyn std::error::Error>> {
        self.recv().map_err(std::convert::Into::into)
    }
}
