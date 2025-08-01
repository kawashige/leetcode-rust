pub struct Solution {}

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let mut result = 0;
        for i in 0..s.len() {
            let mut count = [0; 2];
            for j in (0..=i).rev() {
                let l = (s.as_bytes()[j] - b'0') as usize;
                count[l] += 1;
                if k < count[0] && k < count[1] {
                    break;
                }
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3258() {
        assert_eq!(
            Solution::count_k_constraint_substrings("10101".to_string(), 1),
            12
        );
        assert_eq!(
            Solution::count_k_constraint_substrings("10101".to_string(), 2),
            25
        );
        assert_eq!(
            Solution::count_k_constraint_substrings("11111".to_string(), 1),
            15
        );
    }
}
