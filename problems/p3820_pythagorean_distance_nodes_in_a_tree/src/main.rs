pub struct Solution {}

impl Solution {
    pub fn special_nodes(n: i32, edges: Vec<Vec<i32>>, x: i32, y: i32, z: i32) -> i32 {
        let n = n as usize;
        let mut list = vec![vec![]; n];
        for e in edges {
            list[e[0] as usize].push(e[1] as usize);
            list[e[1] as usize].push(e[0] as usize);
        }

        let p = vec![x as usize, y as usize, z as usize];
        let mut d = vec![vec![-1; n]; 3];

        for i in 0..p.len() {
            let mut stack = vec![(p[i], 0)];

            while let Some((j, dist)) = stack.pop() {
                if d[i][j] != -1 {
                    continue;
                }
                d[i][j] = dist;
                for k in &list[j] {
                    if d[i][*k] == -1 {
                        stack.push((*k, dist + 1));
                    }
                }
            }
        }

        (0..n)
            .filter(|i| {
                let mut dists = vec![d[0][*i], d[1][*i], d[2][*i]];
                dists.sort_unstable();
                dists[0] * dists[0] + dists[1] * dists[1] == dists[2] * dists[2]
            })
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3820() {
        assert_eq!(
            Solution::special_nodes(4, vec![vec![0, 1], vec![0, 2], vec![0, 3]], 1, 2, 3),
            3
        );
        assert_eq!(
            Solution::special_nodes(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]], 0, 3, 2),
            0
        );
        assert_eq!(
            Solution::special_nodes(4, vec![vec![0, 1], vec![1, 2], vec![1, 3]], 1, 3, 0),
            1
        );
    }
}

fn main() {}
