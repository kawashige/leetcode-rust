pub struct Solution {}

impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        for d in (2..colors.len()).rev() {
            for i in 0..colors.len() - d {
                if colors[i] != colors[i + d] {
                    return d as i32;
                }
            }
        }

        1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2078() {
        assert_eq!(Solution::max_distance(vec![1, 1, 6, 1, 1, 1]), 3);
        assert_eq!(Solution::max_distance(vec![1, 8, 3, 8, 3]), 4);
        assert_eq!(Solution::max_distance(vec![0, 1]), 1);
    }
}
