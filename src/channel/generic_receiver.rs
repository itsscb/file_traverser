#[allow(dead_code)]
pub trait GenericReceiver<T> {
    /// Receives a value of type `T`.
    ///
    /// # Errors
    ///
    /// This function will return an error if there is a problem receiving the value.
    fn recv(&self) -> Result<T, Box<dyn std::error::Error>>;
}
