pub struct Solution {}

impl Solution {
    pub fn min_domino_rotations(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let mut a0_count = vec![a.len(); 2];
        let mut b0_count = vec![a.len(); 2];
        let mut a0_enable = true;
        let mut b0_enable = true;
        for i in 0..a.len() {
            if a0_enable && a[i] != a[0] && b[i] != a[0] {
                a0_enable = false;
                a0_count.clear();
            }
            if b0_enable && a[i] != b[0] && b[i] != b[0] {
                b0_enable = false;
                b0_count.clear();
            }
            if !a0_enable && !b0_enable {
                return -1;
            }
            if a0_enable {
                if a[i] == a[0] {
                    a0_count[0] -= 1;
                }
                if b[i] == a[0] {
                    a0_count[1] -= 1;
                }
            }
            if b0_enable {
                if a[i] == b[0] {
                    b0_count[0] -= 1;
                }
                if b[i] == b[0] {
                    b0_count[1] -= 1;
                }
            }
        }
        *a0_count.iter().chain(b0_count.iter()).min().unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day19() {
        assert_eq!(
            2,
            Solution::min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2])
        );
        assert_eq!(
            -1,
            Solution::min_domino_rotations(vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4])
        );
        assert_eq!(
            0,
            Solution::min_domino_rotations(vec![3, 3, 3, 3, 3], vec![3, 3, 3, 3, 4])
        );
        assert_eq!(0, Solution::min_domino_rotations(vec![3, 3], vec![3, 4]));
    }
}
