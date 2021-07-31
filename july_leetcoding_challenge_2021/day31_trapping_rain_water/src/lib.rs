pub struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut r = 0;
        let mut stack: Vec<(usize, i32)> = Vec::new();

        for i in 0..height.len() {
            while !stack.is_empty() && stack.last().unwrap().1 <= height[i] {
                let (_, h) = stack.pop().unwrap();
                if let Some((k, h2)) = stack.last() {
                    r += (height[i].min(*h2) - h) * (i - k - 1) as i32;
                }
            }
            stack.push((i, height[i]));
        }

        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day31() {
        assert_eq!(Solution::trap(vec![8, 2, 8, 9, 0, 1, 7, 7, 9]), 27);
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
