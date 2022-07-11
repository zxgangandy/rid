use rbatis::rbatis::Rbatis;

pub trait  UidConfig {
// GetDB get db handler
    fn GetDB(&self)-> &Rbatis;

// GetPort get port
    fn GetPort(&self) -> &String;

// GetTimeBits get time bits
    fn GetTimeBits(&self) -> i32;

// GetWorkerBits get worker bits
    fn GetWorkerBits(&self) -> i32;

// GetSeqBits get sequence bits
    fn GetSeqBits(&self) -> i32;

// GetEpochSeconds get seconds of the epoch time
    fn GetEpochSeconds(&self) -> i64;

// GetMaxBackwardSeconds get max backward seconds
    fn GetMaxBackwardSeconds(&self) -> i64;

// EnableBackward get enable backward status
    fn EnableBackward(&self) -> bool;
}

// DefaultUidConfig the default uid configure
struct DefaultUidConfig{
    DB:                 Rbatis, // db handler
    Port:               String ,  // app port
    TimeBits:           i32 ,  // time bits
    WorkerBits:         i32 ,  // worker bits
    SeqBits:            i32 ,  // sequence bits
    EpochSeconds:       i64 ,   // epoch seconds
    MaxBackwardSeconds: i64 ,   // max backward seconds
    IsEnableBackward:   bool,     // enable clock backward
}

// New create a default uid configure instance
fn new(db: Rbatis, port: String)-> &DefaultUidConfig {
    return &DefaultUidConfig{
        DB:                 db,
        Port:               port,
        TimeBits:           30,
        WorkerBits:         7,
        SeqBits:            13,
        EpochSeconds:       1550592000000 / 1000,
        MaxBackwardSeconds: 1,
        IsEnableBackward:   true,
    };
}

impl UidConfig for DefaultUidConfig {
    fn GetDB(&self) -> &Rbatis {
        return &self.DB
    }

    fn GetPort(&self) -> &String {
        return &self.Port
    }

    fn GetTimeBits(&self) -> i32 {
        return self.TimeBits
    }

    fn GetWorkerBits(&self) -> i32 {
        return self.WorkerBits
    }

    fn GetSeqBits(&self) -> i32 {
        return self.SeqBits;
    }

    fn GetEpochSeconds(&self) -> i64 {
        self.EpochSeconds
    }

    fn GetMaxBackwardSeconds(&self) -> i64 {
        self.MaxBackwardSeconds
    }

    fn EnableBackward(&self) -> bool {
        self.IsEnableBackward
    }
}