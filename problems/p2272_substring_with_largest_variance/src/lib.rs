pub struct Solution {}

impl Solution {
    pub fn largest_variance(s: String) -> i32 {
        let mut result = 0;
        let bytes = s.as_bytes().iter().fold([false; 26], |mut acc, b| {
            acc[(b - b'a') as usize] = true;
            acc
        });

        for i in 0..bytes.len() {
            for j in 0..bytes.len() {
                if i == j || !bytes[i] || !bytes[j] {
                    continue;
                }

                let mut min_diff = 0;
                let mut min_diff_tmp = 0;
                let mut diff = 0;
                let mut found = [false; 2];

                for b in s.as_bytes() {
                    if b - b'a' == i as u8 {
                        diff += 1;
                        found[0] = true;
                    } else if b - b'a' == j as u8 {
                        min_diff = min_diff_tmp;
                        diff -= 1;
                        min_diff_tmp = min_diff_tmp.min(diff);
                        found[1] = true;
                    }
                    if found[0] && found[1] {
                        result = result.max(diff - min_diff);
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2272() {
        assert_eq!(Solution::largest_variance("bbc".to_string()), 1);
        assert_eq!(Solution::largest_variance("aababbb".to_string()), 3);
        assert_eq!(Solution::largest_variance("abcde".to_string()), 0);
    }
}
