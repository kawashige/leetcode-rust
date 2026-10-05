pub struct Solution {}

impl Solution {
    pub fn first_matching_index(s: String) -> i32 {
        for i in 0..s.len() / 2 {
            if s.as_bytes()[i] == s.as_bytes()[s.len() - 1 - i] {
                return i as i32;
            }
        }

        if s.len() % 2 == 1 {
            s.len() as i32 / 2
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3884() {
        assert_eq!(Solution::first_matching_index("abcacbd".to_string()), 1);
        assert_eq!(Solution::first_matching_index("abc".to_string()), 1);
        assert_eq!(Solution::first_matching_index("abcdab".to_string()), -1);
    }
}

fn main() {}
