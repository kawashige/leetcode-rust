pub struct Solution {}

impl Solution {
    pub fn dfs(i: usize, list: &Vec<Vec<usize>>, flowers: &mut Vec<i32>) {
        if let Some(j) = (1..5).find(|j| list[i].iter().all(|k| flowers[*k] != *j as i32)) {
            flowers[i] = j as i32
        }
        for j in &list[i] {
            if flowers[*j] == 0 {
                Self::dfs(*j, list, flowers);
            }
        }
    }

    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        let mut list = vec![vec![]; n as usize];
        for path in paths {
            list[path[0] as usize - 1].push(path[1] as usize - 1);
            list[path[1] as usize - 1].push(path[0] as usize - 1);
        }

        let mut flowers = vec![0; n as usize];

        for i in 0..(n as usize) {
            if flowers[i] == 0 {
                Self::dfs(i, &list, &mut flowers);
            }
        }

        flowers
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1042() {
        assert_eq!(
            Solution::garden_no_adj(3, vec![vec![1, 2], vec![2, 3], vec![3, 1]]),
            vec![1, 2, 3]
        );
        assert_eq!(
            Solution::garden_no_adj(4, vec![vec![1, 2], vec![3, 4]]),
            vec![1, 2, 1, 2]
        );
        assert_eq!(
            Solution::garden_no_adj(
                4,
                vec![
                    vec![1, 2],
                    vec![2, 3],
                    vec![3, 4],
                    vec![4, 1],
                    vec![1, 3],
                    vec![2, 4]
                ]
            ),
            vec![1, 2, 3, 4]
        );
    }
}
