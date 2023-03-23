pub struct Solution {}

impl Solution {
    pub fn add_rungs(rungs: Vec<i32>, dist: i32) -> i32 {
        (rungs[0] + dist - 1) / dist - 1
            + rungs
                .windows(2)
                .map(|r| (r[1] - r[0] + dist - 1) / dist - 1)
                .sum::<i32>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1936() {
        assert_eq!(Solution::add_rungs(vec![4, 8, 12, 16], 3), 4);
        assert_eq!(Solution::add_rungs(vec![1, 3, 5, 10], 2), 2);
        assert_eq!(Solution::add_rungs(vec![3, 6, 8, 10], 3), 0);
        assert_eq!(Solution::add_rungs(vec![3, 4, 6, 7], 2), 1);
    }
}
