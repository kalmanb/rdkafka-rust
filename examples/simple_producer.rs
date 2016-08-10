
extern crate rdkafka;
use rdkafka::producer::{Producer, ProducerConfig, Record};


fn main() {
    let topic_name = "test-topic".to_string();
    let brokers = vec!["localhost:9042".to_string()];
    let config = ProducerConfig::new(topic_name, brokers)
        .ack_timeout_seconds(23)
        .build();

    println!("{:?}", config);

    let producer = Producer::new(config);

    let record = Record {
        key: "key".to_string(),
        payload: "data".to_string(),
    };

    producer.send(record);
}
