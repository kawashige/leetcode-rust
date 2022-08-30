pub struct Solution {}

impl Solution {
    pub fn is_ok(nums: &[(i32, usize)], l: usize, r: usize) -> bool {
        let mut prev_num = None;
        let mut diff = None;

        for (num, i) in nums {
            if !(l..=r).contains(i) {
                continue;
            }
            if let Some(p) = prev_num {
                if let Some(d) = diff {
                    if num - p != d {
                        return false;
                    }
                } else {
                    diff = Some(num - p)
                }
            }
            prev_num = Some(*num);
        }
        true
    }

    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut nums = nums
            .into_iter()
            .enumerate()
            .map(|(i, num)| (num, i))
            .collect::<Vec<_>>();
        nums.sort_unstable();

        l.into_iter()
            .zip(r.into_iter())
            .map(|(l, r)| Self::is_ok(&nums, l as usize, r as usize))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1630() {
        assert_eq!(
            Solution::check_arithmetic_subarrays(
                vec![4, 6, 5, 9, 3, 7],
                vec![0, 0, 2],
                vec![2, 3, 5]
            ),
            vec![true, false, true]
        );
        assert_eq!(
            Solution::check_arithmetic_subarrays(
                vec![-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10],
                vec![0, 1, 6, 4, 8, 7],
                vec![4, 4, 9, 7, 9, 10]
            ),
            vec![false, true, false, false, true, true]
        );
    }
}
