pub struct Solution {}

impl Solution {
    pub fn is_ok(i: usize, nums: &[i32], queries: &Vec<Vec<i32>>) -> bool {
        let mut acc = vec![0; nums.len() + 1];
        for j in 0..i {
            acc[queries[j][0] as usize] += queries[j][2];
            acc[queries[j][1] as usize + 1] -= queries[j][2];
        }
        for i in 0..nums.len() {
            if 0 < i {
                acc[i] += acc[i - 1];
            }
            if acc[i] < nums[i] {
                return false;
            }
        }
        true
    }

    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        if !Self::is_ok(queries.len(), &nums, &queries) {
            return -1;
        }
        if nums.iter().all(|n| n == &0) {
            return 0;
        }
        let mut ng = 0;
        let mut ok = queries.len();

        while ng + 1 < ok {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, &nums, &queries) {
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
    fn test_3356() {
        assert_eq!(
            Solution::min_zero_array(
                vec![2, 0, 2],
                vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]]
            ),
            2
        );
        assert_eq!(
            Solution::min_zero_array(vec![4, 3, 2, 1], vec![vec![1, 3, 2], vec![0, 2, 1]]),
            -1
        );
    }
}
