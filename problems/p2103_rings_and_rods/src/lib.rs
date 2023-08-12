pub struct Solution {}

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut colors = [0; 10];

        for i in 0..rings.len() / 2 {
            colors[(rings.as_bytes()[2 * i + 1] - b'0') as usize] |= 1
                << match rings.as_bytes()[2 * i] {
                    b'R' => 0,
                    b'G' => 1,
                    b'B' => 2,
                    _ => unreachable!(),
                };
        }

        colors.into_iter().filter(|c| c == &0b111).count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2103() {
        assert_eq!(Solution::count_points("B0B6G0R6R0R6G9".to_string()), 1);
        assert_eq!(Solution::count_points("B0R0G0R9R0B0G0".to_string()), 1);
        assert_eq!(Solution::count_points("G4".to_string()), 0);
    }
}
