pub struct Solution {}

impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        let mut arr = arr;
        arr.sort_unstable();
        arr[0] = 1;

        for i in 1..arr.len() {
            arr[i] = arr[i].min(arr[i - 1] + 1);
        }

        *arr.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1846() {
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![2, 2, 1, 2, 1]),
            2
        );
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![100, 1, 1000]),
            3
        );
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![1, 2, 3, 4, 5]),
            5
        );
    }
}
