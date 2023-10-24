pub struct Solution {}

impl Solution {
    pub fn dig_artifacts(n: i32, artifacts: Vec<Vec<i32>>, dig: Vec<Vec<i32>>) -> i32 {
        let mut grid = vec![vec![false; n as usize]; n as usize];

        for d in dig {
            grid[d[0] as usize][d[1] as usize] = true;
        }

        let mut count = 0;
        for artifact in artifacts {
            let mut uncovered = true;
            for i in artifact[0]..=artifact[2] {
                for j in artifact[1]..=artifact[3] {
                    if !grid[i as usize][j as usize] {
                        uncovered = false;
                        break;
                    }
                }
                if !uncovered {
                    break;
                }
            }
            if uncovered {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2201() {
        assert_eq!(
            Solution::dig_artifacts(
                2,
                vec![vec![0, 0, 0, 0], vec![0, 1, 1, 1]],
                vec![vec![0, 0], vec![0, 1]]
            ),
            1
        );
        assert_eq!(
            Solution::dig_artifacts(
                2,
                vec![vec![0, 0, 0, 0], vec![0, 1, 1, 1]],
                vec![vec![0, 0], vec![0, 1], vec![1, 1]]
            ),
            2
        );
    }
}
