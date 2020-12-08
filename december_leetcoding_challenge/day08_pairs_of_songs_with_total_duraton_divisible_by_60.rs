pub struct Solution {}

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut remains = [0; 60];
        let mut result = 0;
        for t in time {
            let remain = t as usize % 60;
            result += remains[(60 - remain) % 60];
            remains[remain] += 1;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day08() {
        assert_eq!(
            3,
            Solution::num_pairs_divisible_by60(vec![30, 20, 150, 100, 40])
        );
        assert_eq!(3, Solution::num_pairs_divisible_by60(vec![60, 60, 60]));
    }
}
