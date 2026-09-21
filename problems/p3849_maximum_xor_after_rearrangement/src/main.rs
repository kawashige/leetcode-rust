pub struct Solution {}

impl Solution {
    pub fn maximum_xor(s: String, t: String) -> String {
        let mut count = t.as_bytes().iter().filter(|b| b == &&b'1').count();
        if count == 0 {
            return s;
        }
        let mut rearranged = vec![0; t.len()];
        if 0 < count {
            for i in 0..rearranged.len() {
                if s.as_bytes()[i] == b'0' {
                    rearranged[i] = 1;
                    count -= 1;
                    if count == 0 {
                        break;
                    }
                }
            }
        }
        if 0 < count {
            for i in (0..rearranged.len()).rev() {
                if s.as_bytes()[i] == b'1' {
                    rearranged[i] = 1;
                    count -= 1;
                    if count == 0 {
                        break;
                    }
                }
            }
        }

        (0..s.len())
            .map(|i| (((s.as_bytes()[i] - b'0') ^ rearranged[i]) + b'0') as char)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3849() {
        assert_eq!(
            Solution::maximum_xor("101".to_string(), "011".to_string()),
            "110".to_string()
        );
        assert_eq!(
            Solution::maximum_xor("0110".to_string(), "1110".to_string()),
            "1101".to_string()
        );
        assert_eq!(
            Solution::maximum_xor("0101".to_string(), "1001".to_string()),
            "1111".to_string()
        );
    }
}

fn main() {}
