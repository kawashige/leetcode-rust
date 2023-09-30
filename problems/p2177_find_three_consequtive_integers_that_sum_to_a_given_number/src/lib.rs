pub struct Solution {}

impl Solution {
    pub fn sum_of_three(num: i64) -> Vec<i64> {
        if num % 3 == 0 {
            vec![num / 3 - 1, num / 3, num / 3 + 1]
        } else {
            Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2177() {
        assert_eq!(Solution::sum_of_three(33), vec![10, 11, 12]);
        assert_eq!(Solution::sum_of_three(4), vec![]);
    }
}
