use crate::{
    data::{DataGenerator, Message},
    time::Clock,
};
use anyhow::Result;
use std::time::Duration;
use tokio_stream::StreamExt;
use tracing::{error, info, instrument};

pub struct Consumer<C: Clock> {
    pub name: String,
    pub clock: C,
    pub rx_channel: tokio::sync::mpsc::Receiver<Message>,
}

impl<C: Clock> Consumer<C> {
    #[instrument(skip_all, fields(name = self.name))]
    pub async fn consume(mut self) -> Result<()> {
        loop {
            while let Some(message) = self.rx_channel.recv().await {
                self.clock.update_clock(message.time);
                info!("received {} at time {}", message.data, message.time);
            }
        }
    }
}

pub struct Producer<C, T>
where
    C: Clock,
    T: DataGenerator,
{
    pub name: String,
    pub clock: C,
    pub tx_channel: tokio::sync::mpsc::Sender<Message>,
    pub data_source: T,
}

impl<C, T> Producer<C, T>
where
    C: Clock,
    T: DataGenerator,
{
    #[instrument(skip_all, fields(name = self.name))]
    pub async fn produce(mut self) -> Result<()> {
        let mut tmp = self.data_source.generate().next().await.into_iter();
        while let Some(x) = tmp.next() {
            let message = Message {
                time: self.clock.advance_clock(),
                data: x.to_string(),
            };

            info!("sending {} at time {}", &message.data, message.time);

            match self.tx_channel.send(message).await {
                Ok(_) => {}
                Err(e) => error!("Error sending message: {e:?}"),
            };

            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        Ok(())
    }
}
