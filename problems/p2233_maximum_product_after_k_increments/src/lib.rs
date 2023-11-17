pub struct Solution {}

impl Solution {
    pub fn is_ok(mid: i32, k: i32, nums: &[i32]) -> bool {
        let mut d = 0;
        for i in 0..nums.len() {
            if nums[i] < mid {
                d += mid - nums[i];
                if k < d {
                    return false;
                }
            }
        }
        true
    }

    pub fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
        const M: i64 = 1_000_000_007;

        let mut nums = nums;
        nums.sort_unstable();

        let mut ok = 0;
        let mut ng = nums[nums.len() - 1] + k + 1;

        while ok + 1 < ng {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, k, &nums) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        let mut remains = k;
        for i in 0..nums.len() {
            if nums[i] < ok {
                remains -= ok - nums[i];
                nums[i] = ok;
            } else {
                break;
            }
        }

        let mut result = 1;
        for i in 0..nums.len() {
            if 0 < remains {
                nums[i] += 1;
                remains -= 1;
            }
            result *= nums[i] as i64;
            result %= M;
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2233() {
        assert_eq!(
            Solution::maximum_product(vec![24, 5, 64, 53, 26, 38], 54),
            180820950
        );
        assert_eq!(Solution::maximum_product(vec![0, 4], 5), 20);
        assert_eq!(Solution::maximum_product(vec![6, 3, 3, 2], 2), 216);
    }
}
