pub struct Solution {}

impl Solution {
    pub fn is_rectangle_overlap(mut rec1: Vec<i32>, mut rec2: Vec<i32>) -> bool {
        if [0, 1]
            .iter()
            .any(|i| rec1[*i] == rec1[i + 2] || rec2[*i] == rec2[i + 2])
        {
            return false;
        }
        if rec1[0] > rec2[0] {
            std::mem::swap(&mut rec1, &mut rec2);
        }
        rec2[0] < rec1[2] && rec2[3] > rec1[1] && rec2[1] < rec1[3]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0836() {
        assert!(!Solution::is_rectangle_overlap(
            vec![-1, 0, 1, 1],
            vec![0, -1, 0, 1]
        ));
        assert!(Solution::is_rectangle_overlap(
            vec![0, 0, 2, 2],
            vec![1, 1, 3, 3]
        ));
        assert!(!Solution::is_rectangle_overlap(
            vec![0, 0, 1, 1],
            vec![1, 0, 2, 1]
        ));
        assert!(!Solution::is_rectangle_overlap(
            vec![0, 0, 1, 1],
            vec![2, 2, 3, 3]
        ));
    }
}
