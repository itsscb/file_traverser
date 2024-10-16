pub trait GenericSender<T> {
    /// Sends a message of type `T`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the message cannot be sent.
    fn send(&self, msg: T) -> Result<(), Box<dyn std::error::Error>>;
}
