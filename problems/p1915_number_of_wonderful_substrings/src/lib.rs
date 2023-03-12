pub struct Solution {}

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut odd_letter = 0;
        let mut states = vec![0; 2_usize.pow(10) + 1];
        states[0] = 1;

        let mut count = 0;

        for b in word.as_bytes() {
            odd_letter ^= 1 << (b - b'a') as usize;
            for i in 0..states.len() {
                if (i ^ odd_letter).count_ones() < 2 {
                    count += states[i];
                }
            }
            states[odd_letter] += 1;
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1915() {
        assert_eq!(Solution::wonderful_substrings("aba".to_string()), 4);
        assert_eq!(Solution::wonderful_substrings("aabb".to_string()), 9);
        assert_eq!(Solution::wonderful_substrings("he".to_string()), 2);
    }
}
