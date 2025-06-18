use std::pin::Pin;
use tokio_stream::{self as stream, Stream};

pub struct Message {
    pub time: i64,
    pub data: String,
}

pub trait DataGenerator {
    fn generate(&self) -> Pin<Box<dyn Stream<Item = String> + Send>>;
}

#[derive(Clone, Debug)]
pub struct SimpleGenerator;

impl DataGenerator for SimpleGenerator {
    fn generate(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        let iter = (1..10).into_iter().map(|s| s.to_string());
        let stream = stream::iter(iter);
        Box::pin(stream)
    }
}
