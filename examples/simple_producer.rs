extern crate rdkafka;
use rdkafka::producer::{KafkaConfigBuilder, Producer, Record, TopicConfig};

fn main() {
    let topic_name = "test-topic".to_string();
    let brokers = vec!["localhost:9042".to_string()];

    let kafka_config = KafkaConfigBuilder::new(topic_name, brokers)
        .ack_timeout_seconds(23)
        .build();

    println!("{:?}", kafka_config);

    let topic_config = TopicConfig::new(kafka_config);

    let producer = Producer::new(topic_config);

    let record = Record {
        key: "key".to_string(),
        payload: "data".to_string(),
    };

    match producer.send(record) {
        Ok(_) => println!("Sent!"),
        Err(err) => println!("Error: {}", err),
    }
}
