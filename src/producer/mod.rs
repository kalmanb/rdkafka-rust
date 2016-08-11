use std;

#[derive(Debug)]
pub struct Producer {
    pub config: TopicConfig,
}

impl Producer {
    pub fn new(config: TopicConfig) -> Producer {
        Producer { config: config }
    }

    pub fn send<T>(&self, record: Record<T>) -> Result<Record<T>, String>
        where T: std::fmt::Debug
    {
        println!("sending data {:?}", record);
        Ok(record)
    }
}

#[derive(Debug)]
pub struct KafkaConfig {
    topic_name: String,
    brokers: Vec<String>,
    ack_timeout_seconds: u32, /* .topic_name
                               * .topic_partitioner */
}
impl Drop for KafkaConfig {
    fn drop(&mut self) {
        debug!("Cleaning up KafkaConfig");
    }
}
pub struct KafkaConfigBuilder {
    topic_name: String,
    brokers: Vec<String>,
    ack_timeout_seconds: u32, /* .topic_name
                               * .topic_partitioner */
}
// TODO - might want to use a builder to keep the data safe?
impl KafkaConfigBuilder {
    pub fn new(topic_name: String, brokers: Vec<String>) -> Self {
        KafkaConfigBuilder {
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
    pub fn build(self) -> KafkaConfig {
        KafkaConfig {
            topic_name: self.topic_name,
            brokers: self.brokers,
            ack_timeout_seconds: self.ack_timeout_seconds, // .topic_partitioner
        }
    }
}

#[derive(Debug)]
pub struct TopicConfig {
    pub config: KafkaConfig,
}

impl TopicConfig {
    pub fn new(config: KafkaConfig) -> TopicConfig {
        TopicConfig { config: config }
    }
}
impl Drop for TopicConfig {
    fn drop(&mut self) {
        debug!("Cleaning up TopicConfig");
    }
}

#[derive(Debug)]
pub struct Record<T>
    where T: std::fmt::Debug
{
    pub key: T,
    pub payload: T,
}


// trait Partitioner {}
