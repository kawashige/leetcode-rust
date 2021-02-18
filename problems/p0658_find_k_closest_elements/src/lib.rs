pub struct Solution {}

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut s = 0;
        let mut e = arr.len() - 1;

        while (k as usize) < e - s + 1 {
            if (arr[s] - x).abs() <= (arr[e] - x).abs() {
                e -= 1;
            } else {
                s += 1;
            }
        }

        arr[s..=e].to_vec()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0658() {
        assert_eq!(Solution::find_closest_elements(vec![1], 1, 3), vec![1]);
        assert_eq!(
            Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3),
            vec![1, 2, 3, 4]
        );
        assert_eq!(
            Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, -1),
            vec![1, 2, 3, 4]
        );
    }
}
