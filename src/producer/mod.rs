

pub struct Producer {
    // config: ProducerConfig,
}

impl Producer {
    pub fn send(&self, data: String) -> Result<(), String> {
        Ok(())
    }
}

pub struct ProducerBuilder {
    hosts: Vec<String>,
    config: ProducerConfig,
}
impl ProducerBuilder {
    pub fn new() -> ProducerBuilder {
        ProducerBuilder {
            hosts: Vec::new(),
            config: ProducerConfig::new(),
        }
    }
    pub fn hosts(&mut self, hosts: Vec<String>) -> &mut ProducerBuilder {
        self.hosts = hosts;
        self
    }
    pub fn build(&self) -> Producer {
        Producer{
            // config: &self.config,
        }
    }
}


struct ProducerConfig {
    hosts: Vec<String>, /* acks: Acks,
                         * retries: u8,
                         * batch_size: u32,
                         * partitioner: Partitioner, */
}
impl ProducerConfig {
    fn new() -> ProducerConfig {
        ProducerConfig { hosts: Vec::new() }
    }
}

enum Acks {
    // All,
    // num: int,
}

trait Partitioner {}
