pub struct Solution {}

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut list = vec![vec![]; n as usize];
        for road in roads {
            list[road[0] as usize - 1].push((road[1] as usize - 1, road[2]));
            list[road[1] as usize - 1].push((road[0] as usize - 1, road[2]));
        }
        let mut stack = vec![0];
        let mut visited = vec![false; n as usize];
        let mut score = std::i32::MAX;

        while let Some(city) = stack.pop() {
            if visited[city] {
                continue;
            }
            visited[city] = true;
            for (next, distance) in &list[city] {
                if !visited[*next] {
                    stack.push(*next);
                    score = score.min(*distance);
                }
            }
        }

        score
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2492() {
        assert_eq!(
            Solution::min_score(
                4,
                vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5], vec![1, 4, 7]]
            ),
            5
        );
        assert_eq!(
            Solution::min_score(4, vec![vec![1, 2, 2], vec![1, 3, 4], vec![3, 4, 7]]),
            2
        );
    }
}
