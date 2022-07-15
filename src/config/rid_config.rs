
// DefaultUidConfig the default uid configure
#[derive(Clone, Debug)]
pub struct UidConfig {
    pub port:                 String ,  // application server port
    pub time_bits:            i32 ,  // time bits
    pub worker_bits:          i32 ,  // worker bits
    pub seq_bits:             i32 ,  // sequence bits
    pub epoch_seconds:        i64 ,   // epoch seconds
    pub max_backward_seconds: i64 ,   // max backward seconds
    pub enable_backward:      bool,     // enable clock backward
}

impl UidConfig {
    // New create a default uid configure instance
    pub fn new(port: String)-> Self {
        UidConfig {
            port                 ,
            time_bits:           30,
            worker_bits:         7,
            seq_bits:            13,
            epoch_seconds:       1550592000000 / 1000,
            max_backward_seconds: 1,
            enable_backward:   true,
        }
    }
}

