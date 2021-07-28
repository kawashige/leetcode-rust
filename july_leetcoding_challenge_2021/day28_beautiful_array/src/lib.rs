pub struct Solution {}

impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let mut r = vec![1];
        while r.len() < n as usize {
            r = r
                .iter()
                .map(|x| 2 * x - 1)
                .chain(r.iter().map(|x| 2 * x))
                .collect::<Vec<_>>();
        }
        r.into_iter().filter(|x| x <= &n).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day28() {
        assert_eq!(Solution::beautiful_array(4), vec![2, 1, 4, 3]);
        assert_eq!(Solution::beautiful_array(5), vec![3, 1, 2, 5, 4]);
    }
}
