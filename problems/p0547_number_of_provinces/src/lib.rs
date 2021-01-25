pub struct Solution {}

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        fn recurse(is_connected: &Vec<Vec<i32>>, visited: &mut Vec<bool>, i: usize) {
            for j in 0..is_connected[i].len() {
                if is_connected[i][j] == 1 && !visited[j] {
                    visited[j] = true;
                    recurse(is_connected, visited, j);
                }
            }
        }

        let mut visited = vec![false; is_connected.len()];
        let mut count = 0;
        for i in 0..visited.len() {
            if !visited[i] {
                count += 1;
                visited[i] = true;
                recurse(&is_connected, &mut visited, i);
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0547() {
        assert_eq!(
            Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]),
            2
        );
        assert_eq!(
            Solution::find_circle_num(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
            3
        );
    }
}
