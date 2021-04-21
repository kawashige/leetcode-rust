pub struct Solution {}

impl Solution {
    pub fn num_subarray_bounded_max(a: Vec<i32>, l: i32, r: i32) -> i32 {
        a.into_iter()
            .fold((0, 0, 0), |(len, num, count), n| {
                if (l..=r).contains(&n) {
                    (0, num + len + 1, count + len + 1 + num)
                } else if n < l {
                    (len + 1, num, count + num)
                } else {
                    (0, 0, count)
                }
            })
            .2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0795() {
        assert_eq!(
            Solution::num_subarray_bounded_max(vec![2, 1, 4, 3], 2, 3),
            3
        );
        assert_eq!(
            Solution::num_subarray_bounded_max(vec![73, 55, 36, 5, 55, 14, 9, 7, 72, 52], 32, 69),
            22
        );
    }
}
