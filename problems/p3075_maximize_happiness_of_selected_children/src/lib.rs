pub struct Solution {}

impl Solution {
    pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        let mut happiness = happiness;
        happiness.sort_unstable();
        let mut result = 0;

        for i in 0..k as usize {
            if happiness[happiness.len() - 1 - i] <= i as i32 {
                break;
            }
            result += (happiness[happiness.len() - 1 - i] - i as i32) as i64;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3075() {
        assert_eq!(Solution::maximum_happiness_sum(vec![1, 2, 3], 2), 4);
        assert_eq!(Solution::maximum_happiness_sum(vec![1, 1, 1, 1], 2), 1);
        assert_eq!(Solution::maximum_happiness_sum(vec![2, 3, 4, 5], 1), 5);
    }
}
