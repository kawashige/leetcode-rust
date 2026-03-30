pub struct Solution {}

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let mut left = Vec::new();
        let mut right = Vec::new();
        let mut right_amount = 0;
        let mut left_amount = 0;
        let mut init_amount = 0;

        for i in 0..fruits.len() {
            if fruits[i][0] < start_pos && start_pos <= fruits[i][0] + k {
                left.push(fruits[i].clone());
                left_amount += fruits[i][1]
            } else if start_pos < fruits[i][0] && fruits[i][0] <= start_pos + k {
                right.push(fruits[i].clone());
                right_amount += fruits[i][1];
            } else if start_pos == fruits[i][0] {
                init_amount += fruits[i][1];
            }
        }

        let mut result = init_amount + right_amount.max(left_amount);

        if !right.is_empty() && !left.is_empty() {
            // 左に行ってから右
            let mut amount = init_amount + right_amount;
            let mut walk = right[right.len() - 1][0] - start_pos;
            let mut j = right.len() - 1;

            for i in (0..left.len()).rev() {
                amount += left[i][1];
                walk += (if i == left.len() - 1 {
                    start_pos
                } else {
                    left[i + 1][0]
                } - left[i][0])
                    * 2;
                while k < walk && 0 < j {
                    walk -= right[j][0] - right[j - 1][0];
                    amount -= right[j][1];
                    j -= 1;
                }
                if k < walk {
                    break;
                }
                result = result.max(amount);
            }

            // 右に行ってから左
            let mut amount = init_amount + left_amount;
            let mut walk = start_pos - left[0][0];
            let mut j = 0;

            for i in 0..right.len() {
                amount += right[i][1];
                walk += (right[i][0] - if i == 0 { start_pos } else { right[i - 1][0] }) * 2;
                while k < walk && j < left.len() - 1 {
                    walk -= left[j + 1][0] - left[j][0];
                    amount -= left[j][1];
                    j += 1;
                }
                if k < walk {
                    break;
                }
                result = result.max(amount);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2106() {
        assert_eq!(
            Solution::max_total_fruits(
                vec![
                    vec![0, 7],
                    vec![7, 4],
                    vec![9, 10],
                    vec![12, 6],
                    vec![14, 8],
                    vec![16, 5],
                    vec![17, 8],
                    vec![19, 4],
                    vec![20, 1],
                    vec![21, 3],
                    vec![24, 3],
                    vec![25, 3],
                    vec![26, 1],
                    vec![28, 10],
                    vec![30, 9],
                    vec![31, 6],
                    vec![32, 1],
                    vec![37, 5],
                    vec![40, 9]
                ],
                21,
                30
            ),
            71
        );
        assert_eq!(
            Solution::max_total_fruits(vec![vec![2, 8], vec![6, 3], vec![8, 6]], 5, 4),
            9
        );
        assert_eq!(
            Solution::max_total_fruits(
                vec![
                    vec![0, 9],
                    vec![4, 1],
                    vec![5, 7],
                    vec![6, 2],
                    vec![7, 4],
                    vec![10, 9]
                ],
                5,
                4
            ),
            14
        );
        assert_eq!(
            Solution::max_total_fruits(vec![vec![0, 3], vec![6, 4], vec![8, 5]], 3, 2),
            0
        );
    }
}
