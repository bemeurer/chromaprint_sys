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
    fn test_rand_short() {
        let mut rng = ChaChaRng::seed_from_u64(0xdeadbeef);
        let fingerprint;
        const sample_rate: u32 = 44100;
        const channel_count: u32 = 2;
        const duration_secs: u32 = 5;
        unsafe {
            let ctx = chromaprint_new(ChromaprintAlgorithm_CHROMAPRINT_ALGORITHM_DEFAULT as i32);
            check!(chromaprint_start(
                ctx,
                sample_rate as i32,
                channel_count as i32
            ));
            let data: Vec<i16> = (0..(channel_count * duration_secs * sample_rate))
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

    #[test]
    fn test_rand_long() {
        let mut rng = ChaChaRng::seed_from_u64(0xdeadbeef);
        let fingerprint;
        const sample_rate: u32 = 96000;
        const channel_count: u32 = 2;
        const duration_secs: u32 = 30;
        unsafe {
            let ctx = chromaprint_new(ChromaprintAlgorithm_CHROMAPRINT_ALGORITHM_DEFAULT as i32);
            check!(chromaprint_start(
                ctx,
                sample_rate as i32,
                channel_count as i32
            ));
            let data: Vec<i16> = (0..(channel_count * duration_secs * sample_rate))
                .map(|_| rng.gen())
                .collect();
            check!(chromaprint_feed(ctx, data.as_ptr(), data.len() as i32));
            check!(chromaprint_finish(ctx));
            let mut fp: *mut i8 = std::ptr::null_mut();
            check!(chromaprint_get_fingerprint(ctx, &mut fp as *mut *mut i8));
            fingerprint = std::ffi::CString::from_raw(fp).into_string().unwrap();
            chromaprint_free(ctx);
        }
        let expected = "AQAA3UmSJJGUKEkSKQkOC8dd-AEuHBeMHzgMngDEFieew7iGQwRv4LpxGJfw48dpHLCg4zdOiDhw-Phx-PgB3AIPXsdfGIef4jCO77CO_zAA_8BTwDe-4zCOH8cPd3BxHLzw4zK-HfAh9gB9XPBxnLwgHz4OHh_hDxf84ziHH9Sh34B_4BB_AL4OXIR_-JLx4zD-wHqF4ziBF_4A4xdOHDj46oCF4_BlHBR--PgP4_CF49APiMcffPBxXDwOkzguHD9OwAdO_DgMvLCg43h1_Dg-XMYhHocP-HiOHz5xHM-IS8Lh3zigE8cJ6hDw4_hhHqdBQiyA4zQAM4YSVhznjDHAmScEckgY6SRBwFy3EXMXAeGMEQgp0xEQRgihjBJGHICcsRYoB6RQignzjRFWGIeUE046xLQywhmhhBNKCQSREchZwxVizolgnDrOOCYQMMadZbQRyCBhsDVPGCOeFc4pgwyCzgoHLVMGOiekO2IIIxVyGgFnpHPIAQ";
        assert_eq!(&fingerprint, expected);
    }
}
