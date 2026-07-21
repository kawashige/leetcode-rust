pub struct Solution {}

impl Solution {
    pub fn min_cost(s: String, cost: Vec<i32>) -> i64 {
        let mut total_cost = 0;
        let mut char_cost = [0; 26];
        for i in 0..s.len() {
            total_cost += cost[i] as i64;
            char_cost[(s.as_bytes()[i] - b'a') as usize] += cost[i] as i64;
        }

        (0..26).map(|i| total_cost - char_cost[i]).min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3784() {
        assert_eq!(
            Solution::min_cost("aabaac".to_string(), vec![1, 2, 3, 4, 1, 10]),
            11
        );
        assert_eq!(Solution::min_cost("abc".to_string(), vec![10, 5, 8]), 13);
        assert_eq!(
            Solution::min_cost("zzzzz".to_string(), vec![67, 67, 67, 67, 67]),
            0
        );
    }
}

fn main() {}
