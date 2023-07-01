pub struct Solution {}

impl Solution {
    pub fn recurse(i: usize, cookies: &[i32], children: &mut Vec<i32>, unfairness: &mut i32) {
        if i == cookies.len() {
            *unfairness = std::cmp::min(*unfairness, *children.iter().max().unwrap());
            return;
        }

        for j in 0..children.len() {
            if &(cookies[i] + children[j]) < unfairness {
                children[j] += cookies[i];
                Self::recurse(i + 1, cookies, children, unfairness);
                children[j] -= cookies[i];
            }
        }
    }

    pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
        let mut unfairness = std::i32::MAX;
        Self::recurse(0, &cookies, &mut vec![0; k as usize], &mut unfairness);
        unfairness
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2305() {
        assert_eq!(Solution::distribute_cookies(vec![8, 15, 10, 20, 8], 2), 31);
        assert_eq!(
            Solution::distribute_cookies(vec![6, 1, 3, 2, 2, 4, 1, 2], 3),
            7
        );
    }
}
