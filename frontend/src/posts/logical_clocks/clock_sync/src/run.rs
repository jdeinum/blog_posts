use crate::{
    data::{DataGenerator, Message},
    node::{Consumer, Producer},
    time::Clock,
};
use anyhow::Result;
use tokio::task::JoinHandle;

pub async fn run<
    C: Clock + Clone + Send + Sync + 'static,
    T: DataGenerator + Send + Sync + 'static + Clone,
>(
    clock: C,
    generator: T,
    num_producers: usize,
) -> Result<()> {
    // create our channel
    let (tx, rx) = tokio::sync::mpsc::channel::<Message>(10);

    // create our join handles
    let mut handles: Vec<JoinHandle<Result<()>>> = Vec::new();

    // create the consumer
    let conusmer = Consumer {
        name: "consumer".to_string(),
        clock: clock.clone(),
        rx_channel: rx,
    };

    // start consuming
    handles.push(tokio::spawn(conusmer.consume()));

    // create producers
    for x in 0..num_producers {
        let producer = Producer {
            name: format!("producer_{x}"),
            clock: clock.clone(),
            tx_channel: tx.clone(),
            data_source: generator.clone(),
        };
        handles.push(tokio::spawn(producer.produce()));
    }

    Ok(())
}
