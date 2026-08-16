pub struct Solution {}

impl Solution {
    pub fn is_ok(mid: usize, nums: &[i32]) -> bool {
        nums.iter()
            .map(|n| (*n as usize + mid - 1) / mid)
            .sum::<usize>()
            <= mid * mid
    }

    pub fn minimum_k(nums: Vec<i32>) -> i32 {
        let mut ng = 0;
        let mut ok = nums.len() * *nums.iter().max().unwrap() as usize;

        while ng + 1 < ok {
            let mid = (ng + ok) / 2;
            if Self::is_ok(mid, &nums) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3824() {
        assert_eq!(Solution::minimum_k(vec![3, 7, 5]), 3);
        assert_eq!(Solution::minimum_k(vec![1]), 1);
    }
}

fn main() {}
