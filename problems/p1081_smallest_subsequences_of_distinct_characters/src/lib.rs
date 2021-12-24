pub struct Solution {}

impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let bits = s
            .chars()
            .rev()
            .scan(0, |bit, c| {
                *bit = *bit | 1 << (c as usize - 0x61);
                Some(*bit)
            })
            .collect::<Vec<_>>();
        let bits: Vec<usize> = bits.into_iter().rev().collect();
        let bytes = s.as_bytes();

        let mut i = 0;
        let mut s = String::new();
        let mut target = bits[0];
        while 0 < target.count_ones() {
            'outer: for j in 0..26 {
                if target & 1 << j == 0 {
                    continue;
                }

                for k in i..bits.len() {
                    if bytes[k] as usize - 0x61 == j && bits[k] & target == target {
                        s.push((j as u8 + 0x61) as char);
                        target ^= 1 << j;
                        i = k + 1;
                        break 'outer;
                    }
                }
            }
        }

        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1081() {
        assert_eq!(
            Solution::smallest_subsequence("bcabc".to_string()),
            "abc".to_string()
        );
        assert_eq!(
            Solution::smallest_subsequence("cbacdcbc".to_string()),
            "acdb".to_string()
        );
    }
}
