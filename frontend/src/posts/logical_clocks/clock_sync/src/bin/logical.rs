use anyhow::{Context, Result};
use clock_sync_lib::{data::SimpleGenerator, time::Clock};
use std::cmp::max;

#[derive(Clone, Debug)]
struct LamportClock {
    pub time: i64,
}

impl Clock for LamportClock {
    /// Whenever an event occurs on this node (message received, state changed, etc), we
    /// immediately increment our clock to show the passing of time
    fn advance_clock(&mut self) -> i64 {
        self.time += 1;
        self.time
    }

    /// Whenever we receive a message from another node, we set our clock to the maximum of our own
    /// clock and the message timestamp. This ensures time always moves forward, and is what allows
    /// us to define partial ordering for particular events.
    fn update_clock(&mut self, message_timestamp: i64) -> i64 {
        self.time = max(self.time, message_timestamp);
        self.advance_clock()
    }
}

impl LamportClock {
    pub fn new() -> Self {
        Self { time: 0 }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // create clock all nodes will use
    let clock = LamportClock::new();

    // create data source for producers
    let data = SimpleGenerator {};

    clock_sync_lib::run::run(clock, data, 3)
        .await
        .context("run system")?;

    Ok(())
}
