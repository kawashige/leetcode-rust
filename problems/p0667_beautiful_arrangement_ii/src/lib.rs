pub struct Solution {}

impl Solution {
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut e = n;
        let mut s = 1;
        let mut results = Vec::with_capacity(n as usize);
        while results.len() < k {
            results.push(e);
            if results.len() == k {
                for i in (s..e).rev() {
                    results.push(i);
                }
                break;
            }
            e -= 1;
            results.push(s);
            s += 1;
            if results.len() == k {
                for i in s..=e {
                    results.push(i);
                }
                break;
            }
        }
        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_667() {
        assert_eq!(Solution::construct_array(3, 1), vec![3, 2, 1]);
        assert_eq!(Solution::construct_array(3, 2), vec![3, 1, 2]);
        assert_eq!(Solution::construct_array(2, 1), vec![2, 1]);
    }
}
