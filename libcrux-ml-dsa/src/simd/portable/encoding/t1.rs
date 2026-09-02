use crate::{
    constants::BITS_IN_UPPER_PART_OF_T, helper::cloop, simd::portable::vector_type::Coefficients,
};

#[inline(always)]
#[cfg_attr(hax, hax_lib::requires(serialized.len() == 10))]
pub fn serialize(simd_unit: &Coefficients, serialized: &mut [u8]) {
    #[cfg(not(eurydice))]
    debug_assert!(serialized.len() == 10);

    cloop! {
        for (i, coefficients) in simd_unit.values.chunks_exact(4).enumerate() {
            #[cfg(hax)]
            hax_lib::loop_invariant!(|_i: usize| serialized.len() == 10);

            serialized[5 * i] = (coefficients[0] & 0xFF) as u8;
            serialized[5 * i + 1] =
                ((coefficients[1] & 0x3F) as u8) << 2 | ((coefficients[0] >> 8) & 0x03) as u8;
            serialized[5 * i + 2] =
                ((coefficients[2] & 0x0F) as u8) << 4 | ((coefficients[1] >> 6) & 0x0F) as u8;
            serialized[5 * i + 3] =
                ((coefficients[3] & 0x03) as u8) << 6 | ((coefficients[2] >> 4) & 0x3F) as u8;
            serialized[5 * i + 4] = ((coefficients[3] >> 2) & 0xFF) as u8;
        }
    }
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::requires(serialized.len() == 10))]
pub fn deserialize(serialized: &[u8], simd_unit: &mut Coefficients) {
    #[cfg(not(eurydice))]
    debug_assert!(serialized.len() == 10);

    let mask = (1 << BITS_IN_UPPER_PART_OF_T) - 1;

    cloop! {
        for (i, bytes) in serialized.chunks_exact(5).enumerate() {
            #[cfg(hax)]
            hax_lib::loop_invariant!(|_i: usize| serialized.len() == 10);

            let byte0 = bytes[0] as i32;
            let byte1 = bytes[1] as i32;
            let byte2 = bytes[2] as i32;
            let byte3 = bytes[3] as i32;
            let byte4 = bytes[4] as i32;

            simd_unit.values[4 * i] = (byte0 | (byte1 << 8)) & mask;
            simd_unit.values[4 * i + 1] = ((byte1 >> 2) | (byte2 << 6)) & mask;
            simd_unit.values[4 * i + 2] = ((byte2 >> 4) | (byte3 << 4)) & mask;
            simd_unit.values[4 * i + 3] = ((byte3 >> 6) | (byte4 << 2)) & mask;
        }
    }
}
