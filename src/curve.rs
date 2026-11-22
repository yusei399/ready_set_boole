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

/// Decode a value returned by map.
///
/// Panics if the value is outside [0, 1) or between two grid values.
pub fn reverse_map(n: f64) -> (u16, u16) {
    assert!((0.0..1.0).contains(&n), "curve value must be in [0, 1)");
    let scaled = n * INDEX_SCALE;
    let index = scaled as u32;
    assert_eq!(
        scaled,
        f64::from(index),
        "curve value must be an integer multiple of 2^-32"
    );

    let mut x = 0u16;
    let mut y = 0u16;
    for bit in 0..u16::BITS {
        x |= (((index >> (2 * bit)) & 1) as u16) << bit;
        y |= (((index >> (2 * bit + 1)) & 1) as u16) << bit;
    }
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_four_points_follow_z_order() {
        for (index, (x, y)) in [(0, 0), (1, 0), (0, 1), (1, 1)].into_iter().enumerate() {
            let n = index as f64 / INDEX_SCALE;
            assert_eq!(map(x, y), n);
            assert_eq!(reverse_map(n), (x, y));
        }
    }

    #[test]
    fn each_coordinate_bit_maps_to_its_own_index_bit() {
        for bit in 0..u16::BITS {
            let coordinate = 1u16 << bit;
            for (point, index) in [
                ((coordinate, 0), 1u32 << (2 * bit)),
                ((0, coordinate), 1u32 << (2 * bit + 1)),
            ] {
                let n = f64::from(index) / INDEX_SCALE;
                assert_eq!(map(point.0, point.1), n);
                assert_eq!(reverse_map(n), point);
            }
        }
    }

    #[test]
    fn preserves_both_round_trips_at_boundaries() {
        for index in [0, 1, u32::MAX / 2, u32::MAX - 1, u32::MAX] {
            let n = f64::from(index) / INDEX_SCALE;
            let (x, y) = reverse_map(n);
            assert_eq!(map(x, y), n);
        }
        for point in [(0, u16::MAX), (u16::MAX, 0), (u16::MAX, u16::MAX)] {
            assert_eq!(reverse_map(map(point.0, point.1)), point);
        }
        assert_eq!(map(u16::MAX, u16::MAX), 1.0 - 1.0 / INDEX_SCALE);
    }

    #[test]
    #[should_panic(expected = "curve value must be in [0, 1)")]
    fn rejects_upper_endpoint_outside_image() {
        reverse_map(1.0);
    }

    #[test]
    #[should_panic(expected = "curve value must be an integer multiple of 2^-32")]
    fn rejects_values_between_grid_points() {
        reverse_map(0.5 / INDEX_SCALE);
    }
}
