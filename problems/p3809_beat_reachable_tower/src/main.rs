pub struct Solution {}

impl Solution {
    pub fn best_tower(towers: Vec<Vec<i32>>, center: Vec<i32>, radius: i32) -> Vec<i32> {
        let mut towers = towers;
        towers.sort_unstable_by(|a, b| b[2].cmp(&a[2]).then(a[0].cmp(&b[0])).then(a[1].cmp(&b[1])));

        for i in 0..towers.len() {
            if (towers[i][0] - center[0]).abs() + (towers[i][1] - center[1]).abs() <= radius {
                return vec![towers[i][0], towers[i][1]];
            }
        }

        vec![-1, -1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3809() {
        assert_eq!(
            Solution::best_tower(
                vec![vec![1, 2, 5], vec![2, 1, 7], vec![3, 1, 9]],
                vec![1, 1],
                2
            ),
            vec![3, 1]
        );
        assert_eq!(
            Solution::best_tower(
                vec![vec![1, 3, 4], vec![2, 2, 4], vec![4, 4, 7]],
                vec![0, 0],
                5
            ),
            vec![1, 3]
        );
        assert_eq!(
            Solution::best_tower(vec![vec![5, 6, 8], vec![0, 3, 5]], vec![1, 2], 1),
            vec![-1, -1]
        );
    }
}

fn main() {}
