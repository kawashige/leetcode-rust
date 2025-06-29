pub struct Solution {}

impl Solution {
    pub fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32 {
        let mut x = Vec::with_capacity(points.len());
        let mut y = Vec::with_capacity(points.len());
        for i in 0..points.len() {
            x.push((points[i][0].abs(), i));
            y.push((points[i][1].abs(), i));
        }
        x.sort_unstable_by_key(|p| -p.0);
        y.sort_unstable_by_key(|p| -p.0);

        let mut result = 0;
        let mut is_inside = [false; 26];

        while !x.is_empty() || !y.is_empty() {
            let l = x
                .last()
                .unwrap_or(&(std::i32::MAX, 0))
                .0
                .min(y.last().unwrap_or(&(std::i32::MAX, 0)).0);

            let mut tmp = 0;
            let mut valid = true;

            while !x.is_empty() && x.last().unwrap().0 <= l {
                let p = x.pop().unwrap();
                if l < points[p.1][0].abs() || l < points[p.1][1].abs() {
                    continue;
                }
                if is_inside[(s.as_bytes()[p.1] - b'a') as usize] {
                    valid = false;
                    break;
                }
                is_inside[(s.as_bytes()[p.1] - b'a') as usize] = true;
                tmp += 1;
            }

            while !y.is_empty() && y.last().unwrap().0 <= l {
                let p = y.pop().unwrap();
                if l < points[p.1][0].abs()
                    || l < points[p.1][1].abs()
                    || points[p.1][0].abs() == points[p.1][1].abs()
                {
                    continue;
                }
                if is_inside[(s.as_bytes()[p.1] - b'a') as usize] {
                    valid = false;
                    break;
                }
                is_inside[(s.as_bytes()[p.1] - b'a') as usize] = true;
                tmp += 1;
            }

            if !valid {
                break;
            }
            result += tmp;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3143() {
        assert_eq!(
            Solution::max_points_inside_square(
                vec![
                    vec![2, 2],
                    vec![-1, -2],
                    vec![-4, 4],
                    vec![-3, 1],
                    vec![3, -3]
                ],
                "abdca".to_string()
            ),
            2
        );
        assert_eq!(
            Solution::max_points_inside_square(
                vec![vec![1, 1], vec![-2, -2], vec![-2, 2]],
                "abb".to_string()
            ),
            1
        );
        assert_eq!(
            Solution::max_points_inside_square(
                vec![vec![1, 1], vec![-1, -1], vec![2, -2]],
                "ccd".to_string()
            ),
            0
        );
    }
}
