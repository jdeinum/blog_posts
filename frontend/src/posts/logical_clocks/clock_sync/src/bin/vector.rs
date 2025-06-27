use anyhow::Result;
use clock_sync_lib::run::{Message, Node};
use clock_sync_lib::time::Clock;
use std::collections::VecDeque;
use std::{cmp::max, sync::Arc};
use tokio::sync::{Mutex, mpsc};

#[derive(Clone, Debug)]
struct VectorClock {
    pub node_id: usize,
    pub time_vector: Vec<i64>,
}

// NOTE: We clone alot here, which is fine for the example. If we are using shared memory, it would
// be better to wrap messages in an Arc to avoid the overhead of allocating a new vector each time.
impl Clock<Vec<i64>> for VectorClock {
    /// Whenever an event occurs on this node (message received, state changed, etc), we
    /// immediately increment our clock to show the passing of time
    fn advance_clock(&mut self) -> Vec<i64> {
        self.time_vector[self.node_id] += 1;
        self.time_vector.clone()
    }

    /// Whenever we receive a message from another node, we'll update our clock so that each entry
    /// of the vector is the maximum.
    fn update_clock<'a, 'b>(&'a mut self, message_timestamp: &'b Vec<i64>) -> Vec<i64> {
        self.time_vector = self
            .time_vector
            .iter()
            .zip(message_timestamp.iter())
            .map(|(a, b)| max(a, b).clone())
            .collect();
        self.advance_clock()
    }

    fn get_clock(&self) -> Vec<i64> {
        self.time_vector.clone()
    }
}

impl VectorClock {
    pub fn new(node_id: usize, num_nodes: usize) -> Self {
        Self {
            node_id,
            time_vector: (0..num_nodes).map(|_| 0).collect(),
        }
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
        let (tx, rx) = mpsc::channel::<Message<Vec<i64>>>(10);
        send_chans.push(tx.clone());
        receive_chans.push_back(rx);
    }

    // spawn our nodes
    for x in 0..num_nodes {
        let rx = receive_chans.pop_front().unwrap();
        let node = Node {
            clock: Arc::new(Mutex::new(VectorClock::new(x, num_nodes))),
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
