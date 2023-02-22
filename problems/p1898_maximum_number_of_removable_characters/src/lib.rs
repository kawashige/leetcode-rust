pub struct Solution {}

impl Solution {
    pub fn is_ok(s: &str, p: &str, remove_indices: &[usize], mid: usize) -> bool {
        let mut i = 0;

        for j in 0..s.len() {
            if remove_indices[j] < mid {
                continue;
            }
            if i < p.len() && p.as_bytes()[i] == s.as_bytes()[j] {
                i += 1;
            }
        }

        i == p.len()
    }

    pub fn maximum_removals(s: String, p: String, removable: Vec<i32>) -> i32 {
        let mut remove_indices = vec![removable.len(); s.len()];
        for i in 0..removable.len() {
            remove_indices[removable[i] as usize] = i;
        }

        let mut ok = 0;
        let mut ng = removable.len() + 1;

        while ok + 1 < ng {
            let mid = (ok + ng) / 2;
            if Self::is_ok(&s, &p, &remove_indices, mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1898() {
        assert_eq!(
            Solution::maximum_removals("abcacb".to_string(), "ab".to_string(), vec![3, 1, 0]),
            2
        );
        assert_eq!(
            Solution::maximum_removals(
                "abcbddddd".to_string(),
                "abcd".to_string(),
                vec![3, 2, 1, 4, 5, 6]
            ),
            1
        );
        assert_eq!(
            Solution::maximum_removals("abcab".to_string(), "abc".to_string(), vec![0, 1, 2, 3, 4]),
            0
        );
    }
}
