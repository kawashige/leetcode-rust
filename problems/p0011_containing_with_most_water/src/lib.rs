pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut area = (r - l) as i32 * height[r].min(height[l]);

        while l < r {
            if height[r] < height[l] {
                if let Some(new_r) = (l + 1..r).rev().find(|new_r| height[r] < height[*new_r]) {
                    r = new_r;
                } else {
                    break;
                }
            } else {
                if let Some(new_l) = (l + 1..r).find(|new_l| height[l] < height[*new_l]) {
                    l = new_l;
                } else {
                    break;
                }
            }
            area = area.max((r - l) as i32 * height[r].min(height[l]));
        }

        area
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0011() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
        assert_eq!(Solution::max_area(vec![2, 3, 4, 5, 18, 17, 6]), 17);
        assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5, 25, 24, 3, 4]), 24);
    }
}
