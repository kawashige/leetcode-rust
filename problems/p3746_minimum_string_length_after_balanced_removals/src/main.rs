pub struct Solution {}

impl Solution {
    pub fn min_length_after_removals(s: String) -> i32 {
        let mut stack = Vec::new();
        for b in s.as_bytes().iter() {
            if !stack.is_empty() && stack.last() != Some(b) {
                stack.pop();
            } else {
                stack.push(*b)
            }
        }
        stack.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3746() {
        assert_eq!(Solution::min_length_after_removals("aabbab".to_string()), 0);
        assert_eq!(Solution::min_length_after_removals("aaaa".to_string()), 4);
        assert_eq!(Solution::min_length_after_removals("aaabb".to_string()), 1);
    }
}

fn main() {}
