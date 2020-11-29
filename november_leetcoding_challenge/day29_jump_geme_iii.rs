pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut opened = HashSet::new();
        let mut next = vec![start];

        while !next.is_empty() {
            let mut tmp = Vec::new();
            for n in next {
                if arr[n as usize] == 0 {
                    return true;
                }
                opened.insert(n);
                if arr[n as usize] <= n && !opened.contains(&(n - arr[n as usize])) {
                    tmp.push(n - arr[n as usize]);
                }
                if arr[n as usize] + n < arr.len() as i32
                    && !opened.contains(&(n + arr[n as usize]))
                {
                    tmp.push(n + arr[n as usize]);
                }
            }
            next = tmp;
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day29() {
        assert!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5));
        assert!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 0));
        assert!(!Solution::can_reach(vec![3, 0, 2, 1, 2], 2));
    }
}
