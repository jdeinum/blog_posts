use anyhow::Result;
use clock_sync_lib::{
    run::{Message, Node},
    time::Clock,
};
use std::{cmp::max, collections::VecDeque, sync::Arc};
use tokio::sync::{Mutex, mpsc};

#[derive(Clone, Debug)]
struct LamportClock {
    pub time: i64,
}

impl Clock<i64> for LamportClock {
    /// Whenever an event occurs on this node (message received, state changed, etc), we
    /// immediately increment our clock to show the passing of time
    fn advance_clock(&mut self) -> i64 {
        self.time += 1;
        self.time
    }

    /// Whenever we receive a message from another node, we set our clock to the maximum of our own
    /// clock and the message timestamp. This ensures time always moves forward, and is what allows
    /// us to define partial ordering for particular events.
    fn update_clock(&mut self, message_timestamp: &i64) -> i64 {
        self.time = max(self.time, *message_timestamp);
        self.advance_clock()
    }

    fn get_clock(&self) -> i64 {
        self.time
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

    // the handles we'll be waiting on
    let mut join_handles = vec![];

    // create our channels
    let num_nodes = 3;
    let mut receive_chans = VecDeque::new();
    let mut send_chans = vec![];
    for _ in 0..num_nodes {
        let (tx, rx) = mpsc::channel::<Message<i64>>(10);
        send_chans.push(tx.clone());
        receive_chans.push_back(rx);
    }

    // spawn our nodes
    for x in 0..num_nodes {
        let rx = receive_chans.pop_front().unwrap();
        let node = Node {
            clock: Arc::new(Mutex::new(LamportClock::new())),
            node_num: x,
            message_rx: rx,
            nodes: send_chans
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != x)
                .map(|(_, k)| k.clone())
                .collect(),
        };

        join_handles.push(node.run());
    }

    // join all of the tasks
    futures::future::join_all(join_handles).await;

    Ok(())
}
