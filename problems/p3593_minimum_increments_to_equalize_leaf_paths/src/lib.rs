pub struct Solution {}

impl Solution {
    pub fn recurse(
        cur: usize,
        prev: usize,
        cost: &[i32],
        list: &Vec<Vec<usize>>,
        result: &mut i32,
    ) -> i64 {
        let mut childs = Vec::new();
        for child in &list[cur] {
            if child == &prev {
                continue;
            }
            childs.push(Self::recurse(*child, cur, cost, list, result));
        }
        childs.sort_unstable();
        let max = *childs.last().unwrap_or(&0);
        *result += childs.into_iter().filter(|c| c != &max).count() as i32;

        max + cost[cur] as i64
    }

    pub fn min_increase(n: i32, edges: Vec<Vec<i32>>, cost: Vec<i32>) -> i32 {
        let mut list = vec![vec![]; n as usize];
        for e in edges {
            list[e[0] as usize].push(e[1] as usize);
            list[e[1] as usize].push(e[0] as usize);
        }

        let mut result = 0;
        Self::recurse(0, cost.len(), &cost, &list, &mut result);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3593() {
        assert_eq!(
            Solution::min_increase(3, vec![vec![0, 1], vec![0, 2]], vec![2, 1, 3]),
            1
        );
        assert_eq!(
            Solution::min_increase(3, vec![vec![0, 1], vec![1, 2]], vec![5, 1, 4]),
            0
        );
        assert_eq!(
            Solution::min_increase(
                5,
                vec![vec![0, 4], vec![0, 1], vec![1, 2], vec![1, 3]],
                vec![3, 4, 1, 1, 7]
            ),
            1
        );
    }
}
