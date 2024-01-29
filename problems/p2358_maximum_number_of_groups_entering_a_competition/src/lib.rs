pub struct Solution {}

impl Solution {
    pub fn maximum_groups(grades: Vec<i32>) -> i32 {
        let mut groups = 0;
        let mut students = 0;
        for i in 1.. {
            if grades.len() < students + i {
                break;
            }
            students += i;
            groups += 1;
        }
        groups
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2358() {
        assert_eq!(Solution::maximum_groups(vec![10, 6, 12, 7, 3, 5]), 3);
        assert_eq!(Solution::maximum_groups(vec![8, 8]), 1);
    }
}
