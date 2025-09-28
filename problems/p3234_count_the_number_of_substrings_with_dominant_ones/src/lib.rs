pub struct Solution {}

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut zero_indices = Vec::with_capacity(s.len());
        for i in 0..s.len() {
            if s.as_bytes()[i] == b'0' {
                zero_indices.push(i);
            }
        }

        let mut result = 0;
        let mut j = 0;

        for i in 0..s.len() {
            while j < zero_indices.len() && zero_indices[j] < i {
                j += 1;
            }
            if j == zero_indices.len() {
                result += s.len() - i;
                continue;
            }
            result += zero_indices[j] - i;
            for k in j..zero_indices.len() {
                let zeros = k - j + 1;
                let ones = zeros * zeros;
                let min_index = i + zeros + ones - 1;
                if s.len() <= min_index {
                    break;
                }
                let max_index = if k + 1 < zero_indices.len() {
                    zero_indices[k + 1]
                } else {
                    s.len()
                };

                if min_index < max_index {
                    result += max_index - zero_indices[k].max(min_index);
                }
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3234() {
        assert_eq!(Solution::number_of_substrings("00011".to_string()), 5);
        assert_eq!(Solution::number_of_substrings("101101".to_string()), 16);
    }
}
