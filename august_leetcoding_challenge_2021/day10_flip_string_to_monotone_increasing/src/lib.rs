pub struct Solution {}

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let zeros = s.as_bytes().iter().filter(|b| b == &&b'0').count();

        s.as_bytes()
            .iter()
            .enumerate()
            .fold((s.len() - zeros, 0), |(result, ones), (i, b)| {
                (
                    result.min(zeros + ones * 2 - i),
                    if b == &b'1' { ones + 1 } else { ones },
                )
            })
            .0 as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day10() {
        assert_eq!(Solution::min_flips_mono_incr("00110".to_string()), 1);
        assert_eq!(Solution::min_flips_mono_incr("010110".to_string()), 2);
        assert_eq!(Solution::min_flips_mono_incr("00011000".to_string()), 2);
    }
}
