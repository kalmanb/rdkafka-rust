
extern crate rdkafka;
use rdkafka::producer::{Producer, ProducerConfig, Record};


fn main() {
    let config = ProducerConfig::new("test-topic".to_string(), vec!["localhost:9042".to_string()])
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
