pub struct Solution {}

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();
        let mut i = 0;
        let mut j = 0;
        while i < g.len() && j < s.len() {
            if g[i] <= s[j] {
                i += 1;
            }
            j += 1;
        }
        i as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0455() {
        assert_eq!(
            2,
            Solution::find_content_children(vec![10, 9, 8, 7], vec![5, 6, 7, 8])
        );
        assert_eq!(1, Solution::find_content_children(vec![1, 2, 3], vec![3]));
        assert_eq!(0, Solution::find_content_children(vec![3], vec![1]));
        assert_eq!(1, Solution::find_content_children(vec![3], vec![3]));
        assert_eq!(
            1,
            Solution::find_content_children(vec![1, 2, 3], vec![1, 1])
        );
        assert_eq!(
            2,
            Solution::find_content_children(vec![1, 2], vec![1, 2, 3])
        );
    }
}
