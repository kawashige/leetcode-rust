pub struct Solution {}

impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i64 {
        let n = nums.len();
        // represents the occurrence count of prefix sums -n, -(n-1), ..., 0, 1, ..., n, with index offset by n.
        let mut pre = vec![0; n * 2 + 1];
        pre[n] = 1;
        let mut cnt = n as i32;
        let mut ans: i64 = 0;
        let mut presum: i64 = 0;
        for i in 0..n {
            if nums[i] == target {
                presum += pre[cnt as usize] as i64;
                cnt += 1;
                pre[cnt as usize] += 1;
            } else {
                cnt -= 1;
                presum -= pre[cnt as usize] as i64;
                pre[cnt as usize] += 1;
            }
            ans += presum;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3739() {
        assert_eq!(Solution::count_majority_subarrays(vec![1, 2, 2, 3], 2), 5);
        assert_eq!(Solution::count_majority_subarrays(vec![1, 1, 1, 1], 1), 10);
        assert_eq!(Solution::count_majority_subarrays(vec![1, 2, 3], 4), 0);
    }
}

fn main() {}
