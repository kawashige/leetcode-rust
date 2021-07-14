use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let set = arr.clone().into_iter().collect::<HashSet<i32>>();

        let mut max = 0;
        for i in 0..arr.len() {
            for j in (i + 1)..arr.len() {
                let mut a = arr[i];
                let mut b = arr[j];
                let mut l = 0;
                while set.contains(&(a + b)) {
                    l += 1;
                    let tmp = b;
                    b = a + b;
                    a = tmp;
                }

                if l > 0 {
                    max = std::cmp::max(max, l + 2);
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0873() {
        assert_eq!(
            Solution::len_longest_fib_subseq(vec![2, 4, 5, 6, 7, 8, 11, 13, 14, 15, 21, 22, 34]),
            5
        );
        assert_eq!(Solution::len_longest_fib_subseq(vec![1, 3, 7]), 0);
        assert_eq!(
            Solution::len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8]),
            5
        );
        assert_eq!(
            Solution::len_longest_fib_subseq(vec![1, 3, 7, 11, 12, 14, 18]),
            3
        );
    }
}
