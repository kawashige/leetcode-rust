pub struct Solution {}

impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold([0; 2], |mut count, num| {
                if num < 0 {
                    count[0] += 1;
                } else if 0 < num {
                    count[1] += 1;
                }
                count
            })
            .into_iter()
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2529() {
        assert_eq!(Solution::maximum_count(vec![-2, -1, -1, 1, 2, 3]), 3);
        assert_eq!(Solution::maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]), 3);
        assert_eq!(Solution::maximum_count(vec![5, 20, 66, 1314]), 4);
    }
}
