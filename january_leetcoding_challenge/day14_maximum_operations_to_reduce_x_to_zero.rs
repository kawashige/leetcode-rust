pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let l = nums.len();
        let sum = nums.iter().sum();
        let target = sum - x;

        if target == 0 {
            return l as i32;
        } else if target < 0 {
            return -1;
        }

        let mut sums = vec![vec![0; l]; l];
        sums[0][l - 1] = sum;

        for i in (0..(l - 1)).rev() {
            for j in 0..(l - i) {
                let sum = if j == 0 {
                    sums[j][j + i + 1] - nums[j + i + 1]
                } else {
                    sums[j - 1][j + i] - nums[j - 1]
                };
                if sum == target {
                    return (l - 1 - i) as i32;
                }
                sums[j][j + i] = sum;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day14() {
        assert_eq!(2, Solution::min_operations(vec![1, 1, 4, 2, 3], 5));
        assert_eq!(-1, Solution::min_operations(vec![5, 6, 7, 8, 9], 4));
        assert_eq!(5, Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10));
    }
}
