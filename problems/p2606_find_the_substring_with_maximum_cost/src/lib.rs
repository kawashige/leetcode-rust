pub struct Solution {}

impl Solution {
    pub fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32 {
        let mut cost = (1..27).collect::<Vec<_>>();
        for i in 0..chars.len() {
            cost[(chars.as_bytes()[i] - b'a') as usize] = vals[i];
        }

        let mut sum = 0;
        let mut min = 0;
        let mut result = 0;

        for i in 0..s.len() {
            sum += cost[(s.as_bytes()[i] - b'a') as usize];
            result = result.max(sum - min);
            min = min.min(sum);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2606() {
        assert_eq!(
            Solution::maximum_cost_substring("adaa".to_string(), "d".to_string(), vec![-1000]),
            2
        );
        assert_eq!(
            Solution::maximum_cost_substring(
                "abc".to_string(),
                "abc".to_string(),
                vec![-1, -1, -1]
            ),
            0
        );
    }
}
