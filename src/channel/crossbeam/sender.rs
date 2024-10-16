#![cfg(feature = "crossbeam")]
use crate::channel::GenericSender;
use crossbeam_channel::Sender;

#[allow(dead_code)]
impl<T> GenericSender<T> for Sender<T>
where
    T: Send + 'static,
{
    fn send(&self, msg: T) -> Result<(), Box<dyn std::error::Error>> {
        self.send(msg).map_err(std::convert::Into::into)
    }
}
