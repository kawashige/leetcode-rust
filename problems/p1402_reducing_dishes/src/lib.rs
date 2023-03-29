pub struct Solution {}

impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let mut satisfaction = satisfaction;
        satisfaction.sort_unstable();

        let mut result = 0;
        let mut sum = 0;

        for i in (0..satisfaction.len()).rev() {
            if sum + satisfaction[i] <= 0 {
                break;
            }
            sum += satisfaction[i];
            result += sum;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1402() {
        assert_eq!(Solution::max_satisfaction(vec![-1, -8, 0, 5, -9]), 14);
        assert_eq!(Solution::max_satisfaction(vec![4, 3, 2]), 20);
        assert_eq!(Solution::max_satisfaction(vec![-1, -4, -5]), 0);
    }
}
