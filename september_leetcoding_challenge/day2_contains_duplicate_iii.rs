pub struct Solution {}

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        fn check(nums: &Vec<i32>, t: i32) -> bool {
            nums.windows(2)
                .any(|x| (x[0] as i64 - x[1] as i64).abs() <= t as i64)
        }

        if nums.len() < 2 {
            return false;
        }

        let k = std::cmp::min(k, nums.len() as i32 - 1);
        let mut sets = nums[0..=(k as usize)].to_vec();
        sets.sort();
        if check(&sets, t) {
            return true;
        }
        for i in (k as usize + 1)..nums.len() {
            match sets.binary_search(&nums[i - k as usize - 1]) {
                Ok(n) => {
                    sets.remove(n);
                }
                _ => {}
            }
            let pos = sets.binary_search(&nums[i]).unwrap_or_else(|e| e);
            sets.insert(pos, nums[i]);
            if check(&sets, t) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day2() {
        assert!(Solution::contains_nearby_almost_duplicate(
            vec![1, 2, 3, 1],
            3,
            0
        ));
        assert!(Solution::contains_nearby_almost_duplicate(
            vec![1, 0, 1, 1],
            1,
            2
        ));
        assert!(!Solution::contains_nearby_almost_duplicate(
            vec![1, 5, 9, 1, 5, 9],
            2,
            3
        ));
        assert!(!Solution::contains_nearby_almost_duplicate(vec![], 0, 0));
        assert!(Solution::contains_nearby_almost_duplicate(vec![1, 2], 1, 5));
        assert!(!Solution::contains_nearby_almost_duplicate(vec![1], 1, 1));
        assert!(!Solution::contains_nearby_almost_duplicate(
            vec![-1, 2147483647],
            1,
            2147483647
        ));
    }
}
