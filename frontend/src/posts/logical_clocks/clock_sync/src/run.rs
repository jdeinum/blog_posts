use crate::time::Clock;
use anyhow::{Context, Result};
use std::{fmt::Debug, sync::Arc, time::Duration};
use tokio::sync::{Mutex, mpsc};

pub type Message<T> = (usize, T, String);

#[derive(Debug)]
pub struct Node<C, T>
where
    C: Clock<T> + Debug,
{
    pub clock: Arc<Mutex<C>>,
    pub node_num: usize,
    pub message_rx: mpsc::Receiver<Message<T>>,
    pub nodes: Vec<mpsc::Sender<Message<T>>>,
}

async fn listen_for_messages<C, T>(
    mut rx: mpsc::Receiver<Message<T>>,
    clock: Arc<Mutex<C>>,
    node_num: usize,
) where
    C: Clock<T> + Debug,
    T: Debug,
{
    loop {
        match rx.recv().await.context("receive message") {
            Ok(m) => {
                println!(
                    "Node {} | Local Time: {:?} | Received message {} from node {} with timestamp {:?}",
                    node_num,
                    clock.lock().await.get_clock(),
                    m.2,
                    m.0,
                    m.1,
                );

                clock.lock().await.update_clock(&m.1);
            }
            Err(e) => {
                eprintln!("Node {node_num}: Error: {e:?}");
            }
        };
    }
}

impl<C, T> Node<C, T>
where
    T: Debug + Send + Sync + 'static,
    C: Clock<T> + Send + Sync + 'static + Debug,
{
    pub async fn run(self) -> Result<()> {
        // spawn a task that receives messages
        tokio::spawn(listen_for_messages(
            self.message_rx,
            self.clock.clone(),
            self.node_num,
        ));

        for i in 0..10 {
            // now we'll do our action
            match rand::random_range(0..4) {
                // 25 % of the time, we create a local event
                0 => {
                    // new event! advance our clock
                    let t = self.clock.lock().await.advance_clock();

                    // pretend something happend
                    println!(
                        "Node {} | Local Time: {t:?} | Generated local event {i}",
                        self.node_num
                    );
                }

                // 25% of the time, we will send a message to
                1 => {
                    // new event! advance our clock
                    let t = self.clock.lock().await.advance_clock();

                    // determine what node to send to
                    let node_to_send_to = rand::random_range(0..self.nodes.len());

                    // send the message
                    println!(
                        "Node {} | Local time: {t:?} | Sending message {i}",
                        self.node_num
                    );
                    self.nodes[node_to_send_to]
                        .send((self.node_num, t, i.to_string()))
                        .await
                        .context("send message to node")?;
                }

                // 50% of the time we do nothing
                _ => {
                    println!(
                        "Node {} | Local Time: {:?} | Doing nothing",
                        self.node_num,
                        self.clock.lock().await.get_clock()
                    );
                }
            }

            // sleep for a little
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        // sleep for 2 seconds just to allow all nodes to finish up
        // NOTE: This is a hack, not a solution
        tokio::time::sleep(Duration::from_secs(2)).await;

        Ok(())
    }
}
