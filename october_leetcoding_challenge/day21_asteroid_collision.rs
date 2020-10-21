pub struct Solution {}

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        fn recurse(v: &mut Vec<i32>, a: i32) {
            if let Some(last) = v.last() {
                if last < &0 {
                    v.push(a);
                } else if last == &a.abs() {
                    v.pop();
                } else if last < &a.abs() {
                    v.pop();
                    recurse(v, a);
                }
            } else {
                v.push(a);
            }
        }

        let mut results = Vec::new();
        for a in asteroids {
            if a > 0 || results.is_empty() {
                results.push(a);
            } else {
                recurse(&mut results, a);
            }
        }

        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day21() {
        assert_eq!(vec![5, 10], Solution::asteroid_collision(vec![5, 10, -5]));
        assert_eq!(
            vec![] as Vec<i32>,
            Solution::asteroid_collision(vec![8, -8])
        );
        assert_eq!(vec![10], Solution::asteroid_collision(vec![10, 2, -5]));
        assert_eq!(
            vec![-2, -1, 1, 2],
            Solution::asteroid_collision(vec![-2, -1, 1, 2])
        );
    }
}
