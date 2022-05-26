pub struct Solution {}

impl Solution {
    pub fn count(array: &[i32]) -> [i32; 1001] {
        array.iter().fold([0; 1001], |mut count, x| {
            count[*x as usize] += 1;
            count
        })
    }

    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        Self::count(&target)
            .iter()
            .zip(Self::count(&arr))
            .all(|(x1, x2)| x1 == &x2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1460() {
        assert!(Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]));
        assert!(Solution::can_be_equal(vec![7], vec![7]));
        assert!(!Solution::can_be_equal(vec![3, 7, 9], vec![3, 7, 11]));
    }
}
