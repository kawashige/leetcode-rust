pub struct Solution {}

impl Solution {
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut fac = Vec::new();

        let mut x = 1;
        while x * x <= n {
            if n % x == 0 {
                fac.push(x);
                if n / x != x {
                    fac.push(n / x);
                }
            }
            x += 1;
        }

        let mut target = nums.clone();
        target.sort();

        fn kmp(nums1: &[i32], nums2: &[i32]) -> bool {
            let n = nums1.len();
            let m = nums2.len();
            let mut pi = vec![0; m];
            let mut j = 0;

            for i in 1..m {
                while j > 0 && nums2[i] != nums2[j] {
                    j = pi[j - 1];
                }
                if nums2[i] == nums2[j] {
                    j += 1;
                }
                pi[i] = j;
            }

            j = 0;
            for i in 0..n {
                while j > 0 && nums1[i] != nums2[j] {
                    j = pi[j - 1];
                }
                if nums1[i] == nums2[j] {
                    j += 1;
                }
                if j == m {
                    return true;
                }
            }
            false
        }

        fn check(nums1: &[i32], nums2: &[i32]) -> bool {
            // Check whether cyclic rotations of nums1 can produce nums2
            let mut doubled = nums1.to_vec();
            doubled.extend_from_slice(nums1);
            kmp(&doubled, nums2)
        }

        let mut ans = 0;

        for d in fac {
            let mut all_good = true;
            let mut left = 0;
            while left < n {
                let right = left + d - 1;

                if !check(&nums[left..=right], &target[left..=right]) {
                    all_good = false;
                    break;
                }
                left += d;
            }
            if all_good {
                ans += d as i32;
            }
        }

        ans
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3886() {
        assert_eq!(Solution::sortable_integers(vec![3, 1, 2]), 3);
        assert_eq!(Solution::sortable_integers(vec![7, 6, 5]), 0);
        assert_eq!(Solution::sortable_integers(vec![5, 8]), 3);
    }
}

fn main() {}
