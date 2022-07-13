use std::sync::Arc;
use rbatis::rbatis::Rbatis;

pub trait  UidConfig {

// get_port get port
    fn get_port(&self) -> String;

// get_time_bits get time bits
    fn get_time_bits(&self) -> i32;

// get_worker_bits get worker bits
    fn get_worker_bits(&self) -> i32;

// get_seq_bits get sequence bits
    fn get_seq_bits(&self) -> i32;

// get_epoch_seconds get seconds of the epoch time
    fn get_epoch_seconds(&self) -> i64;

// get_max_backward_seconds get max backward seconds
    fn get_max_backward_seconds(&self) -> i64;

// enable_backward get enable backward status
    fn enable_backward(&self) -> bool;
}

// DefaultUidConfig the default uid configure
#[derive(Clone, Debug)]
pub struct DefaultUidConfig{
    port:               String ,  // app port
    time_bits:           i32 ,  // time bits
    worker_bits:         i32 ,  // worker bits
    seq_bits:            i32 ,  // sequence bits
    epoch_seconds:       i64 ,   // epoch seconds
    max_backward_seconds: i64 ,   // max backward seconds
    is_enable_backward:   bool,     // enable clock backward
}

impl DefaultUidConfig {
    // New create a default uid configure instance
    fn new(port: String)-> Self {
        DefaultUidConfig{
            port                 ,
            time_bits:           30,
            worker_bits:         7,
            seq_bits:            13,
            epoch_seconds:       1550592000000 / 1000,
            max_backward_seconds: 1,
            is_enable_backward:   true,
        }
    }
}


impl UidConfig for DefaultUidConfig {
    fn get_port(&self) -> String {
        return self.port.clone()
    }

    fn get_time_bits(&self) -> i32 {
        return self.time_bits
    }

    fn get_worker_bits(&self) -> i32 {
        return self.worker_bits
    }

    fn get_seq_bits(&self) -> i32 {
        return self.seq_bits;
    }

    fn get_epoch_seconds(&self) -> i64 {
        self.epoch_seconds
    }

    fn get_max_backward_seconds(&self) -> i64 {
        self.max_backward_seconds
    }

    fn enable_backward(&self) -> bool {
        self.is_enable_backward
    }
}