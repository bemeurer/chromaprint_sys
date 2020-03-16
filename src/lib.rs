#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use crate::*;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaChaRng;

    macro_rules! check {
        ($f:expr) => {
            // 1 means success to chromaprint for some reason.
            assert_eq!($f, 1);
        };
    }

    #[test]
    fn test_dummy_acoustid() {
        let mut rng = ChaChaRng::seed_from_u64(0xdeadbeef);
        let fingerprint;
        const sample_rate: i32 = 44100;
        const channel_count: i32 = 2;
        const duration: i32 = 5;
        unsafe {
            let ctx = chromaprint_new(ChromaprintAlgorithm_CHROMAPRINT_ALGORITHM_DEFAULT as i32);
            check!(chromaprint_start(ctx, sample_rate, channel_count));
            let data: Vec<i16> = (0..(channel_count * duration * sample_rate))
                .map(|_| rng.gen())
                .collect();
            check!(chromaprint_feed(ctx, data.as_ptr(), data.len() as i32));
            check!(chromaprint_finish(ctx));
            let mut fp: *mut i8 = std::ptr::null_mut();
            check!(chromaprint_get_fingerprint(ctx, &mut fp as *mut *mut i8));
            fingerprint = std::ffi::CString::from_raw(fp).into_string().unwrap();
            chromaprint_free(ctx);
        }
        let expected = "AQAAE5EkSUmSJEqkwMcBHT98wMSP4zAOUMdx3AflhBXCGmeuEmdI";
        assert_eq!(&fingerprint, expected);
    }
}
