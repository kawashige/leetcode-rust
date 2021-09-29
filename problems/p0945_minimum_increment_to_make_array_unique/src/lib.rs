pub struct Solution {}

impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.into_iter()
            .fold((0, -1), |(inc, max), x| {
                if max < x {
                    (inc, x)
                } else {
                    (inc + max + 1 - x, max + 1)
                }
            })
            .0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0945() {
        assert_eq!(Solution::min_increment_for_unique(vec![0]), 0);
        assert_eq!(Solution::min_increment_for_unique(vec![1, 2, 2]), 1);
        assert_eq!(
            Solution::min_increment_for_unique(vec![3, 2, 1, 2, 1, 7]),
            6
        );
    }
}
