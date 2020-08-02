pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        for i in 0..len {
            for j in 0..len {
                if i == j {
                    continue;
                } else if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        return Vec::new();
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_1() {
        let ret = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(0, ret[0]);
        assert_eq!(1, ret[1]);
    }
}
