use std;

#[derive(Debug)]
pub struct Producer {
    pub config: ProducerConfig,
}

impl Producer {
    pub fn new(config: ProducerConfig) -> Producer {
        Producer { config: config }
    }

    pub fn send<T>(&self, record: Record<T>) -> Result<(), String>
        where T: std::fmt::Debug
    {
        println!("sending data {:?}", record);
        Ok(())
    }
}

#[derive(Debug)]
pub struct ProducerConfig {
    topic_name: String,
    brokers: Vec<String>,
    ack_timeout_seconds: u32, /* .topic_name
                               * .topic_partitioner */
}
// TODO - might want to use a builder to keep the data safe?
impl ProducerConfig {
    pub fn new(topic_name: String, brokers: Vec<String>) -> Self {
        ProducerConfig {
            topic_name: topic_name,
            brokers: brokers,
            ack_timeout_seconds: 10, /* Default to 10 seconds
                                      * .topic_partitioner */
        }
    }
    pub fn ack_timeout_seconds(mut self, ack_timeout_seconds: u32) -> Self {
        self.ack_timeout_seconds = ack_timeout_seconds;
        self
    }
    pub fn build(self) -> ProducerConfig {
        ProducerConfig {
            topic_name: self.topic_name,
            brokers: self.brokers,
            ack_timeout_seconds: self.ack_timeout_seconds, // .topic_partitioner
        }
    }
}

#[derive(Debug)]
pub struct Record<T>
    where T: std::fmt::Debug
{
    pub key: T,
    pub payload: T,
}


// enum Acks {
//     // All,
//     // num: int,
// }

// trait Partitioner {}
