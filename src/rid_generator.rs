use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use rbatis::rbatis::Rbatis;
use crate::worker::worker_assigner;
use crate::bits_allocator;
use crate::config::rid_config;
use crate::config::rid_config::UidConfig;

struct DefaultUidGenerator {
    worker_id_assigner: worker_assigner::Assigner,
    bits_allocator:  bits_allocator::BitsAllocator,
    config:          rid_config::DefaultUidConfig,
    worker_id:       i64,
    last_second:     Arc<Mutex<i64>>,
    sequence :       i64,
}


impl DefaultUidGenerator {
    //New create the default uid generator instance
    pub  async fn new(config: rid_config::DefaultUidConfig, RB: Rbatis) -> Self {
        let idAssigner = worker_assigner::Assigner::new(config.clone(), RB);
        let allocator = bits_allocator::BitsAllocator::new(
            config.get_time_bits(),
            config.get_worker_bits(),
            config.get_seq_bits()
        );
        let mut workerId = idAssigner.assign_worker_id().await;

        if workerId > allocator.max_worker_id {
            workerId = workerId % allocator.max_worker_id
        }

        return DefaultUidGenerator {
            worker_id_assigner: idAssigner,
            bits_allocator: allocator,
            config,
            worker_id: workerId,
            last_second: Arc::new(Mutex::new(0)),
            sequence: 0,
        }
    }

    // GetUID generate the unique id
    pub fn GetUID(& mut self) -> i64 {
        let c = &self.config;
        return self.nextId(c.get_epoch_seconds(), c.get_max_backward_seconds(), c.enable_backward())
    }


    fn  nextId(& mut self, epochSeconds: i64, maxBackwardSeconds: i64, enableBackward: bool) -> i64 {
        // g.mutex.Lock()
        // defer g.mutex.Unlock()
        let mut last_second = self.last_second.lock().unwrap();
        let mut  currentSecond = self.get_current_second(epochSeconds);

        if currentSecond < *last_second {
            let refusedSeconds  = *last_second - currentSecond;
            if !enableBackward {
                panic!("Clock moved backwards. Refusing seconds");
            }

            if refusedSeconds <= maxBackwardSeconds {
                while currentSecond < *last_second {
                    currentSecond = self.get_current_second(epochSeconds)
                }
            } else {
                panic!("Clock moved backwards. Refused seconds bigger than max backward seconds")
            }
        }

            // At the same second, increase sequence
        if currentSecond == *last_second {
            self.sequence = (self.sequence + 1) & self.bits_allocator.max_sequence;
            // Exceed the max sequence, we wait the next second to generate uid
            if self.sequence == 0 {
                currentSecond = self.getNextSecond(*last_second, epochSeconds);
            }

            // At the different second, sequence restart from zero
        } else {
            self.sequence = 0;
        }

        *last_second = currentSecond;

        // Allocate bits for UID
        return self.bits_allocator.allocate(currentSecond-epochSeconds, self.worker_id, self.sequence);
    }

    fn get_current_second(&self, epoch_seconds: i64) -> i64 {
        let current_seconds = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        if current_seconds - epoch_seconds > self.bits_allocator.max_delta_seconds {
            panic!("Timestamp bits is exhausted. Refusing UID generate.")
        }

        return current_seconds
    }


    fn getNextSecond(&self, last_timestamp: i64, epoch_seconds: i64) ->i64 {
        let mut timestamp = self.get_current_second(epoch_seconds);

        while timestamp <= last_timestamp {
            timestamp = self.get_current_second(epoch_seconds)
        }

        return timestamp
    }

}

// ParseUID parse the generated unique id then get the meta information
// +------+----------------------+----------------+-----------+
// | sign |     delta seconds    | worker node id | sequence  |
// +------+----------------------+----------------+-----------+
//   1bit          30bits              7bits         13bits
// func (g *DefaultUidGenerator) ParseUID(uid int64) string {
// totalBits := (uint32)(TotalBits)
// signBits := g.bits_allocator.signBits
// timestampBits := g.bits_allocator.timestampBits
// workerIdBits := g.bits_allocator.workerIdBits
// sequenceBits := g.bits_allocator.sequenceBits
// 
// // parse UID
// sequence := (uid << (totalBits - sequenceBits)) >> (totalBits - sequenceBits)
// worker_id := (uid << (timestampBits + signBits)) >> (totalBits - workerIdBits)
// deltaSeconds := uid >> (workerIdBits + sequenceBits)
// 
// // format as string
// return fmt.Sprintf("{\"UID\":\"%d\",\"timestamp\":\"%d\",\"worker_id\":\"%d\",\"sequence\":\"%d\"}",
// uid, g.config.GetEpochSeconds()+deltaSeconds, worker_id, sequence)
// }
// }



