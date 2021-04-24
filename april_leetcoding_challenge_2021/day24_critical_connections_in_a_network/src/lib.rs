pub struct Solution {}

impl Solution {
    pub fn dfs(
        g: &Vec<Vec<usize>>,
        parent: &mut Vec<usize>,
        prenum: &mut Vec<usize>,
        lowest: &mut Vec<usize>,
        visited: &mut Vec<bool>,
        current: usize,
        prev: usize,
        timer: &mut usize,
    ) {
        prenum[current] = *timer;
        lowest[current] = *timer;
        *timer += 1;

        visited[current] = true;

        for i in 0..g[current].len() {
            let next = g[current][i];
            if !visited[next] {
                parent[next] = current;
                Self::dfs(g, parent, prenum, lowest, visited, next, current, timer);
                lowest[current] = std::cmp::min(lowest[current], lowest[next]);
            } else if next != prev {
                lowest[current] = std::cmp::min(lowest[current], prenum[next]);
            }
        }
    }

    fn bridges(g: &Vec<Vec<usize>>) -> Vec<Vec<i32>> {
        let mut visited = vec![false; g.len()];
        let mut parent = vec![0; g.len()];
        let mut prenum = vec![0; g.len()];
        let mut lowest = vec![0; g.len()];
        let mut timer = 1;

        Self::dfs(
            g,
            &mut parent,
            &mut prenum,
            &mut lowest,
            &mut visited,
            0,
            std::usize::MAX,
            &mut timer,
        );

        let mut bridges = Vec::new();
        for i in 1..g.len() {
            let p = parent[i];
            if prenum[p] < lowest[i] {
                bridges.push(vec![p as i32, i as i32]);
            }
        }

        bridges
    }

    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut g = vec![vec![]; n as usize];

        for edge in &connections {
            g[edge[0] as usize].push(edge[1] as usize);
            g[edge[1] as usize].push(edge[0] as usize);
        }

        Self::bridges(&g)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day24() {
        assert_eq!(
            Solution::critical_connections(4, vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]]),
            vec![vec![1, 3]]
        );
    }
}
