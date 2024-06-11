pub struct Solution {}

impl Solution {
    pub fn is_ok(mid: i32, nums: &[i32], k: i32) -> bool {
        let mut stealed = vec![false; nums.len()];
        let mut count = 0;

        for i in 0..nums.len() {
            if mid < nums[i] {
                continue;
            }
            if 0 < i && stealed[i - 1] {
                continue;
            }
            stealed[i] = true;
            count += 1;
        }

        k <= count
    }

    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let mut ng = 0;
        let mut ok = *nums.iter().max().unwrap();

        while ng + 1 < ok {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, &nums, k) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2560() {
        assert_eq!(Solution::min_capability(vec![2, 3, 5, 9], 2), 5);
        assert_eq!(Solution::min_capability(vec![2, 7, 9, 3, 1], 2), 2);
    }
}
