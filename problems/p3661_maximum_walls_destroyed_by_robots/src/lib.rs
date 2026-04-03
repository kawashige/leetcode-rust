use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, walls: Vec<i32>) -> i32 {
        let mut positions = Vec::new();
        for i in 0..robots.len() {
            positions.push((robots[i], 0, distance[i]));
        }
        for i in 0..walls.len() {
            positions.push((walls[i], 1, 0));
        }
        positions.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));

        let mut walls = VecDeque::new();
        let mut left = Vec::new();
        for i in 0..positions.len() {
            if positions[i].1 == 0 {
                // robot
                while !walls.is_empty() && walls[0] < positions[i].0 - positions[i].2 {
                    walls.pop_front();
                }
                left.push((walls.len() as i32, i));
                walls.clear();
            } else {
                // wall
                walls.push_back(positions[i].0);
            }
        }

        println!("positions: {:?}", positions);
        let mut nums = vec![0; left.len()];
        for i in 1..left.len() {
            nums[i] = (left[i].1 - left[i - 1].1 - 1
                + if 0 < left[i - 1].1
                    && positions[left[i - 1].1 - 1].0 == positions[left[i - 1].1].0
                {
                    1
                } else {
                    0
                }) as i32;
        }

        positions.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        let mut walls = VecDeque::new();
        let mut right = Vec::new();
        for i in (0..positions.len()).rev() {
            if positions[i].1 == 0 {
                // robot
                while !walls.is_empty() && positions[i].0 + positions[i].2 < walls[0] {
                    walls.pop_front();
                }
                right.push((walls.len() as i32, i));
                walls.clear();
            } else {
                // wall
                walls.push_back(positions[i].0);
            }
        }
        right = right.into_iter().rev().collect::<Vec<_>>();

        println!("positions: {:?}", positions);
        println!("left: {:?}", left);
        println!("right: {:?}", right);
        println!("nums: {:?}", nums);

        let mut dp = vec![vec![0; 2]; left.len()];
        dp[0][0] = left[0].0;
        dp[0][1] = right[0].0;
        for i in 1..left.len() {
            dp[i][0] = (dp[i - 1][0] + left[i].0)
                .max(dp[i - 1][1] - right[i - 1].0 + (right[i - 1].0 + left[i].0).min(nums[i]));
            dp[i][1] = dp[i - 1][0].max(dp[i - 1][1]) + right[i].0;
        }

        println!("dp: {:?}", dp);

        *dp.last().unwrap().iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3661() {
        assert_eq!(
            Solution::max_walls(
                vec![17, 59, 32, 11, 72, 18],
                vec![5, 7, 6, 5, 2, 10],
                vec![
                    17, 25, 33, 29, 54, 53, 18, 35, 39, 37, 20, 14, 34, 13, 16, 58, 22, 51, 56, 27,
                    10, 15, 12, 23, 45, 43, 21, 2, 42, 7, 32, 40, 8, 9, 1, 5, 55, 30, 38, 4, 3, 31,
                    36, 41, 57, 28, 11, 49, 26, 19, 50, 52, 6, 47, 46, 44, 24, 48
                ]
            ),
            37
        );
        assert_eq!(Solution::max_walls(vec![4], vec![3], vec![1, 10]), 1);
        assert_eq!(
            Solution::max_walls(vec![10, 2], vec![5, 1], vec![5, 2, 7]),
            3
        );
        assert_eq!(Solution::max_walls(vec![1, 2], vec![100, 1], vec![10]), 0);
    }
}
