const INDEX_SCALE: f64 = (1u64 << (2 * u16::BITS)) as f64;

/// Interleave the coordinate bits in Z-order, putting x in even bit positions.
/// The result is the 32-bit index divided by 2^32.
pub fn map(x: u16, y: u16) -> f64 {
    let x = u32::from(x);
    let y = u32::from(y);
    let mut index = 0;
    for bit in 0..u16::BITS {
        index |= ((x >> bit) & 1) << (2 * bit);
        index |= ((y >> bit) & 1) << (2 * bit + 1);
    }

    // All 32-bit integers and their quotients by 2^32 are exact in f64.
    f64::from(index) / INDEX_SCALE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_four_points_follow_z_order() {
        for (index, (x, y)) in [(0, 0), (1, 0), (0, 1), (1, 1)].into_iter().enumerate() {
            let n = index as f64 / INDEX_SCALE;
            assert_eq!(map(x, y), n);
        }
    }
}
