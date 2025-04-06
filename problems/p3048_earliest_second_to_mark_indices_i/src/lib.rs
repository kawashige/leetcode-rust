pub struct Solution {}

impl Solution {
    pub fn is_ok(i: usize, nums: &[i32], change_indices: &[i32]) -> bool {
        let mut seen = vec![false; nums.len() + 1];
        let mut indices = vec![-1; i + 1];
        for j in (0..i).rev() {
            if seen[change_indices[j] as usize] {
                continue;
            }
            seen[change_indices[j] as usize] = true;
            indices[j] = change_indices[j] as i32;
        }
        if (1..seen.len()).any(|i| !seen[i]) {
            return false;
        }
        let mut count = 0;
        for j in 0..i {
            if indices[j] == -1 {
                count += 1;
            } else if nums[indices[j] as usize - 1] <= count {
                count -= nums[indices[j] as usize - 1];
            } else {
                return false;
            }
        }
        true
    }

    pub fn earliest_second_to_mark_indices(nums: Vec<i32>, change_indices: Vec<i32>) -> i32 {
        if !Self::is_ok(change_indices.len(), &nums, &change_indices) {
            return -1;
        }

        let mut ng = 0;
        let mut ok = change_indices.len();

        while ng + 1 < ok {
            let mid = (ng + ok) / 2;
            if Self::is_ok(mid, &nums, &change_indices) {
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
    fn test_3048() {
        assert_eq!(
            Solution::earliest_second_to_mark_indices(vec![2, 2, 0], vec![2, 2, 2, 2, 3, 2, 2, 1]),
            8
        );
        assert_eq!(
            Solution::earliest_second_to_mark_indices(vec![1, 3], vec![1, 1, 1, 2, 1, 1, 1]),
            6
        );
        assert_eq!(
            Solution::earliest_second_to_mark_indices(vec![0, 1], vec![2, 2, 2]),
            -1
        );
    }
}
