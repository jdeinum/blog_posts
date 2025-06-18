pub trait Clock {
    /// Advance the clock by 1 unit
    fn advance_clock(&mut self) -> i64;

    /// Update the clock when receiving a message including the timestamp
    fn update_clock(&mut self, message_timestamp: i64) -> i64;
}
