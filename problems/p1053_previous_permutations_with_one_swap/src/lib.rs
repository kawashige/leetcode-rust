pub struct Solution {}

impl Solution {
    pub fn prev_perm_opt1(mut arr: Vec<i32>) -> Vec<i32> {
        let mut min = std::i32::MAX;

        for i in (0..arr.len()).rev() {
            if min < arr[i] {
                if let Some(j) = ((i + 1)..arr.len())
                    .rev()
                    .filter(|j| arr[*j] < arr[i])
                    .max_by_key(|j| arr[*j])
                {
                    arr.swap(i, j);
                    break;
                }
            }
            min = min.min(arr[i]);
        }

        arr
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1053() {
        assert_eq!(Solution::prev_perm_opt1(vec![3, 2, 1]), vec![3, 1, 2]);
        assert_eq!(Solution::prev_perm_opt1(vec![1, 1, 5]), vec![1, 1, 5]);
        assert_eq!(
            Solution::prev_perm_opt1(vec![1, 9, 4, 6, 7]),
            vec![1, 7, 4, 6, 9]
        );
        assert_eq!(Solution::prev_perm_opt1(vec![3, 1, 1, 3]), vec![1, 3, 1, 3]);
    }
}
