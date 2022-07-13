
const TOTAL_BITS: i32 = 1 << 6;

//bits 分配器
pub struct BitsAllocator {
    pub sign_bits:     i32,
    pub timestamp_bits: i32,
    pub worker_id_bits:  i32,
    pub sequence_bits: i32,

    // Max value for workId & sequence
    pub max_delta_seconds: i64,
    pub max_worker_id:  i64,
    pub max_sequence:   i64,

    //Shift for timestamp & workerId
    pub timestamp_shift: i32,
    pub worker_id_shift:  i32,
}




impl BitsAllocator {
    //构建一个bits管理器实例
    pub fn new(timestamp_bits: i32, worker_id_bits: i32, sequence_bits: i32) -> Self {
        let signBits: i32 = 1;

        let allocateTotalBits = signBits + timestamp_bits + worker_id_bits + sequence_bits;

        if allocateTotalBits > TOTAL_BITS {
            panic!("allocate larger than 64 bits")
        }

        return BitsAllocator {
            sign_bits: signBits,
            timestamp_bits,
            worker_id_bits,
            sequence_bits,
            max_delta_seconds: -1 ^ (-1 << timestamp_bits),
            max_worker_id: -1 ^ (-1 << worker_id_bits),
            max_sequence: -1 ^ (-1 << sequence_bits),
            timestamp_shift: worker_id_bits + sequence_bits,
            worker_id_shift: sequence_bits,
        }
    }

    pub fn allocate(&self, delta_seconds: i64, worker_id: i64, sequence: i64) -> i64 {
        return (delta_seconds << self.timestamp_shift) | (worker_id << self.worker_id_shift) | sequence
    }
}

