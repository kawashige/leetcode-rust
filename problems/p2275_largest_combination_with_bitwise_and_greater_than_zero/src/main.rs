pub struct Solution {}

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut count = [0; 32];

        for candicate in candidates {
            for i in 0..count.len() {
                if candicate & 1 << i != 0 {
                    count[i] += 1;
                }
            }
        }
        count.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2275() {
        assert_eq!(
            Solution::largest_combination(vec![16, 17, 71, 62, 12, 24, 14]),
            4
        );
        assert_eq!(Solution::largest_combination(vec![8, 8]), 2);
    }
}
