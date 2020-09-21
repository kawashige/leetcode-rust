pub struct Solution {}

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut passengers = [0; 1000];
        for t in trips {
            for i in t[1]..t[2] {
                passengers[i as usize] += t[0];
                if capacity < passengers[i as usize] {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day21() {
        assert!(!Solution::car_pooling(
            vec![vec![2, 1, 5], vec![3, 3, 7]],
            4
        ));
        assert!(Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 5));
        assert!(Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 5, 7]], 3));
        assert!(Solution::car_pooling(
            vec![vec![3, 2, 7], vec![3, 7, 9], vec![8, 3, 9]],
            11
        ));
    }
}
