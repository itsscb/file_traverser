#[cfg(feature = "crossbeam")]
mod crossbeam;

#[cfg(not(feature = "crossbeam"))]
mod mpsc;

mod generic_receiver;
mod generic_sender;

pub use generic_receiver::GenericReceiver;
pub use generic_sender::GenericSender;
