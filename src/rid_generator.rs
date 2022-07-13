use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use rbatis::rbatis::Rbatis;

use crate::worker::worker_assigner;
use crate::bits_allocator;
use crate::config::rid_config;
use crate::config::rid_config::UidConfig;

struct UidGenerator {
    worker_id_assigner: worker_assigner::Assigner,
    bits_allocator:  bits_allocator::BitsAllocator,
    config:          rid_config::DefaultUidConfig,
    worker_id:       i64,
    last_second:     Arc<Mutex<i64>>,
    sequence :       i64,
}

impl UidGenerator {
    //New create the default uid generator instance
    pub  async fn new(config: rid_config::DefaultUidConfig, rb: Arc<Rbatis>) -> Self {
        let id_assigner = worker_assigner::Assigner::new(config.clone(), Arc::clone(&rb));
        let allocator = bits_allocator::BitsAllocator::new(
            config.get_time_bits(),
            config.get_worker_bits(),
            config.get_seq_bits()
        );
        let mut worker_id = id_assigner.assign_worker_id().await;

        if worker_id > allocator.max_worker_id {
            worker_id = worker_id % allocator.max_worker_id
        }

        return UidGenerator {
            worker_id_assigner: id_assigner,
            bits_allocator: allocator,
            config,
            worker_id,
            last_second: Arc::new(Mutex::new(0)),
            sequence: 0,
        }
    }

    // get_uid generate the unique id
    pub fn get_uid(& mut self) -> i64 {
        let c = &self.config;
        return self.next_id(c.get_epoch_seconds(), c.get_max_backward_seconds(), c.enable_backward())
    }

    // parse_uid parse the generated unique id then get the meta information
    // +------+----------------------+----------------+-----------+
    // | sign |     delta seconds    | worker node id | sequence  |
    // +------+----------------------+----------------+-----------+
    //   1bit          30bits              7bits         13bits
    fn parse_uid(&self, uid: i64) -> string {
        let total_bits = bits_allocator::TOTAL_BITS;
        let sign_bits = self.bits_allocator.sign_bits;
        let timestamp_bits = self.bits_allocator.timestamp_bits;
        let worker_id_bits = self.bits_allocator.worker_id_bits;
        let sequence_bits = self.bits_allocator.sequence_bits;

        // parse UID
        let sequence = (uid << (total_bits - sequence_bits)) >> (total_bits - sequence_bits);
        let worker_id = (uid << (timestamp_bits + sign_bits)) >> (total_bits - worker_id_bits);
        let delta_seconds = uid >> (worker_id_bits + sequence_bits);

        // format as string
        return format!(r#"{{"uid\":\"{}\",\"timestamp\":\"{}\",\"worker_id\":\"{}\",\"sequence\":\"{}\"}}"#,
                       uid, self.config.get_epoch_seconds() + delta_seconds, worker_id, sequence);
    }

    fn next_id(& mut self, epoch_seconds: i64, max_backward_seconds: i64, enable_backward: bool) -> i64 {
        let mut last_second = self.last_second.lock().unwrap();
        let mut current_second = self.get_current_second(epoch_seconds);

        if current_second < *last_second {
            let refused_seconds = *last_second - current_second;
            if !enable_backward {
                panic!("Clock moved backwards. Refusing seconds");
            }

            if refused_seconds <= max_backward_seconds {
                while current_second < *last_second {
                    current_second = self.get_current_second(epoch_seconds)
                }
            } else {
                panic!("Clock moved backwards. Refused seconds bigger than max backward seconds")
            }
        }

        // At the same second, increase sequence
        if current_second == *last_second {
            self.sequence = (self.sequence + 1) & self.bits_allocator.max_sequence;
            // Exceed the max sequence, we wait the next second to generate uid
            if self.sequence == 0 {
                current_second = self.get_next_second(*last_second, epoch_seconds);
            }

            // At the different second, sequence restart from zero
        } else {
            self.sequence = 0;
        }

        *last_second = current_second;

        // Allocate bits for UID
        return self.bits_allocator.allocate(current_second - epoch_seconds, self.worker_id, self.sequence);
    }

    fn get_current_second(&self, epoch_seconds: i64) -> i64 {
        let current_seconds = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        if current_seconds - epoch_seconds > self.bits_allocator.max_delta_seconds {
            panic!("Timestamp bits is exhausted. Refusing UID generate.")
        }

        return current_seconds
    }

    fn get_next_second(&self, last_timestamp: i64, epoch_seconds: i64) ->i64 {
        let mut timestamp = self.get_current_second(epoch_seconds);

        while timestamp <= last_timestamp {
            timestamp = self.get_current_second(epoch_seconds)
        }

        return timestamp
    }

}





