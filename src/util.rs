mod into_array;

pub use into_array::IntoArray;

static mut XORSHIFT_STATE: u32 = 0;

pub fn srand(seed: u32) {
    unsafe {
        XORSHIFT_STATE = seed;
    }
}

pub fn rand() -> u32 {
    unsafe {
        XORSHIFT_STATE ^= XORSHIFT_STATE >> 17;
        XORSHIFT_STATE ^= XORSHIFT_STATE << 5;
        XORSHIFT_STATE ^= XORSHIFT_STATE << 13;
        XORSHIFT_STATE
    }
}
