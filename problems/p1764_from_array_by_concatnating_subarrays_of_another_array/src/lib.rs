pub struct Solution {}

impl Solution {
    pub fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
        let mut g = 0;
        let mut i = 0;

        while g < groups.len() && i < nums.len() {
            if groups[g][0] == nums[i] {
                let mut is_ok = true;
                for j in 0..groups[g].len() {
                    if nums.len() <= i + j || groups[g][j] != nums[i + j] {
                        is_ok = false;
                        break;
                    }
                }

                if is_ok {
                    i += groups[g].len();
                    g += 1;
                    continue;
                }
            }
            i += 1;
        }

        g == groups.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1764() {
        assert!(Solution::can_choose(
            vec![vec![1, -1, -1], vec![3, -2, 0]],
            vec![1, -1, 0, 1, -1, -1, 3, -2, 0]
        ));
        assert!(!Solution::can_choose(
            vec![vec![10, -2], vec![1, 2, 3, 4]],
            vec![1, 2, 3, 4, 10, -2]
        ));
        assert!(!Solution::can_choose(
            vec![vec![1, 2, 3], vec![3, 4]],
            vec![7, 7, 1, 2, 3, 4, 7, 7]
        ));
    }
}
