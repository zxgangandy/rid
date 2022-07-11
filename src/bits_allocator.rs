
const TOTAL_BITS: i32 = 1 << 6;

//bits 分配器
struct BitsAllocator {
    sign_bits:     i32,
    timestamp_bits: i32,
    worker_id_bits:  i32,
    sequence_bits: i32,

    // Max value for workId & sequence
    max_delta_seconds: i64,
    max_worker_id:  i64,
    max_sequence:   i64,

    //Shift for timestamp & workerId
    timestamp_shift: i32,
    worker_id_shift:  i32,
}

//构建一个bits管理器实例
fn new_bits_allocator(timestamp_bits: i32, worker_id_bits: i32, sequence_bits: i32) -> &BitsAllocator {
    let signBits: i32 = 1;

    let allocateTotalBits = signBits + timestamp_bits + worker_id_bits + sequence_bits;

    if allocateTotalBits > TOTAL_BITS {
        panic!("allocate larger than 64 bits")
    }

    return &BitsAllocator {
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


impl BitsAllocator {
    pub fn allocate(deltaSeconds: i64, workerId: i64, sequence: i64) -> i64 {
        return (deltaSeconds << b.timestampShift) | (workerId << b.workerIdShift) | sequence
    }
}

