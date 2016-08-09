
extern crate rdkafka;
use rdkafka::producer::{Producer, ProducerBuilder};


fn main() {
    let producer = ProducerBuilder::new()
        .hosts(vec!["localhost:9042".to_string()])
        .build();

    let data = "test".to_string();
    producer.send(data);
}
