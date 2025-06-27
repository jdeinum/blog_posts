pub trait Clock<T> {
    /// Advance the clock by 1 unit
    fn advance_clock(&mut self) -> T;

    /// Update the clock when receiving a message including the timestamp
    fn update_clock(&mut self, message_timestamp: &T) -> T;

    /// get the current time
    fn get_clock(&self) -> T;
}
