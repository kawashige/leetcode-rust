pub struct Solution {}

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut remains = vec![0; k as usize * 2];

        for x in arr {
            let m = x % k;
            if 0 < remains[((k - m.abs()) * m.signum() + k) as usize] {
                remains[((k - m.abs()) * m.signum() + k) as usize] -= 1;
            } else if 0 < remains[(k - (m % k)) as usize] {
                remains[(k - (m % k)) as usize] -= 1;
            } else {
                remains[(k + m) as usize] += 1;
            }
        }

        remains.iter().all(|c| &0 == c)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1497() {
        assert!(Solution::can_arrange(vec![-1, 1, -2, 2, -3, 3, -4, 4], 3));
        assert!(Solution::can_arrange(
            vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9],
            5
        ));
        assert!(Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 7));
        assert!(!Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 10));
    }
}
