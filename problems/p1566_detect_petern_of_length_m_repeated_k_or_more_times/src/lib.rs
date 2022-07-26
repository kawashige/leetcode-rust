pub struct Solution {}

impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        if (arr.len() as i32) < m * k {
            return false;
        }

        for i in 0..=(arr.len() - (m * k) as usize) {
            if (1..k as usize)
                .all(|j| (0..m as usize).all(|l| arr[i + l] == arr[i + l + j * m as usize]))
            {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1566() {
        assert!(Solution::contains_pattern(vec![1, 2, 4, 4, 4, 4], 1, 3));
        assert!(Solution::contains_pattern(
            vec![1, 2, 1, 2, 1, 1, 1, 3],
            2,
            2
        ));
        assert!(!Solution::contains_pattern(vec![1, 2, 1, 2, 3], 2, 3));
    }
}
