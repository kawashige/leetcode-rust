pub struct Solution {}

impl Solution {
    pub fn check_child_has_apple(
        node: usize,
        parent: usize,
        list: &Vec<Vec<usize>>,
        has_apple: &Vec<bool>,
        child_has_apple: &mut Vec<bool>,
    ) {
        for child in &list[node] {
            if child == &parent {
                continue;
            }
            Self::check_child_has_apple(*child, node, list, has_apple, child_has_apple);
            child_has_apple[node] |= child_has_apple[*child];
        }
    }

    pub fn traverse(
        node: usize,
        parent: usize,
        list: &Vec<Vec<usize>>,
        child_has_apple: &Vec<bool>,
        time: &mut i32,
    ) {
        for child in &list[node] {
            if child == &parent || !child_has_apple[*child] {
                continue;
            }
            Self::traverse(*child, node, list, child_has_apple, time);
            *time += 2;
        }
    }

    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let mut list = vec![vec![]; n as usize];
        for edge in edges {
            list[edge[0] as usize].push(edge[1] as usize);
            list[edge[1] as usize].push(edge[0] as usize);
        }

        let mut child_has_apple = has_apple.clone();
        Self::check_child_has_apple(0, n as usize, &list, &has_apple, &mut child_has_apple);

        let mut time = 0;
        Self::traverse(0, n as usize, &list, &child_has_apple, &mut time);

        time
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1443() {
        assert_eq!(
            Solution::min_time(
                7,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 4],
                    vec![1, 5],
                    vec![2, 3],
                    vec![2, 6]
                ],
                vec![false, false, true, false, true, true, false]
            ),
            8
        );
        assert_eq!(
            Solution::min_time(
                7,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 4],
                    vec![1, 5],
                    vec![2, 3],
                    vec![2, 6]
                ],
                vec![false, false, true, false, false, true, false]
            ),
            6
        );
        assert_eq!(
            Solution::min_time(
                7,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 4],
                    vec![1, 5],
                    vec![2, 3],
                    vec![2, 6]
                ],
                vec![false, false, false, false, false, false, false]
            ),
            0
        );
    }
}
