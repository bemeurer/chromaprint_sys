use chromaprint_sys::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

macro_rules! check {
    ($f:expr) => {
        // 1 means success to chromaprint for some reason.
        assert_eq!($f, 1);
    };
}

fn main() {
    let mut rng = ChaChaRng::seed_from_u64(0xdeadbeef);
    let sample_rate = 44100;
    let channel_count = 2;
    let duration = 30;
    unsafe {
        let ctx = chromaprint_new(ChromaprintAlgorithm_CHROMAPRINT_ALGORITHM_DEFAULT as i32);
        check!(chromaprint_start(ctx, sample_rate, channel_count));
        for _ in 1..=(channel_count * duration * sample_rate) {
            check!(chromaprint_feed(ctx, &rng.gen() as *const i16, 1));
        }
        check!(chromaprint_finish(ctx));
        let mut fp: *mut i8 = std::ptr::null_mut();
        check!(chromaprint_get_fingerprint(ctx, &mut fp as *mut *mut i8));
        let fp = std::ffi::CStr::from_ptr(fp);
        eprintln!("{}", fp.to_str().unwrap());
        chromaprint_free(ctx);
    }
}
