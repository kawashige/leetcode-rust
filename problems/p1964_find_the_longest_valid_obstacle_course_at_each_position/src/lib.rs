pub struct Solution {}

impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; obstacles.len()];
        let mut stack = Vec::with_capacity(obstacles.len());

        for i in 0..obstacles.len() {
            match stack.binary_search(&(obstacles[i] * 2 + 1)) {
                Err(j) if stack.len() <= j => {
                    stack.push(obstacles[i] * 2);
                    result[i] = stack.len() as i32;
                }
                Err(j) => {
                    stack[j] = obstacles[i] * 2;
                    result[i] = j as i32 + 1;
                }
                _ => unreachable!(),
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1964() {
        assert_eq!(
            Solution::longest_obstacle_course_at_each_position(vec![5, 1, 5, 5, 1, 3, 4, 5, 1, 4]),
            vec![1, 1, 2, 3, 2, 3, 4, 5, 3, 5]
        );
        assert_eq!(
            Solution::longest_obstacle_course_at_each_position(vec![1, 2, 3, 2]),
            vec![1, 2, 3, 3]
        );
        assert_eq!(
            Solution::longest_obstacle_course_at_each_position(vec![2, 2, 1]),
            vec![1, 2, 1]
        );
        assert_eq!(
            Solution::longest_obstacle_course_at_each_position(vec![3, 1, 5, 6, 4, 2]),
            vec![1, 1, 2, 3, 2, 2]
        );
    }
}
