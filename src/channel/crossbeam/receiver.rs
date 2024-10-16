#![cfg(feature = "crossbeam")]
use crate::channel::generic_receiver::GenericReceiver;
use crossbeam_channel::Receiver;

#[allow(dead_code)]
impl<T> GenericReceiver<T> for Receiver<T>
where
    T: Send + 'static,
{
    fn recv(&self) -> Result<T, Box<dyn std::error::Error>> {
        self.recv().map_err(std::convert::Into::into)
    }
}
