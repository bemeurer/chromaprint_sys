#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use crate::*;

    macro_rules! check {
        ($f:expr) => {
            // 1 means success to chromaprint for some reason.
            assert_eq!($f, 1);
        };
    }

    #[test]
    fn test_dummy_acoustid() {
        unsafe {
            let ctx = chromaprint_new(ChromaprintAlgorithm_CHROMAPRINT_ALGORITHM_DEFAULT as i32);
            check!(chromaprint_start(ctx, 44100, 2));
            let sample: i16 = 0;
            check!(chromaprint_feed(ctx, &sample as *const i16, 1));
            check!(chromaprint_finish(ctx));
        }
    }
}
