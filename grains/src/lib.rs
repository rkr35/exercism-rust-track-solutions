const NUM_SQUARES: u32 = 64;

pub fn square(s: u32) -> u64 {
    match s {
        1 ... NUM_SQUARES => 2_u64.pow(s - 1),
        _ => panic!("Square must be between 1 and {}", NUM_SQUARES)
    }
}

pub fn total() -> u64 {
    (2_u128.pow(NUM_SQUARES + 1) - 1) as u64
}
