pub struct Solution {}

impl Solution {
    pub fn check_valid_cuts(n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        let mut rectangles = rectangles;

        // horizontal
        rectangles.sort_unstable_by(|a, b| a[1].cmp(&b[1]).then(a[3].cmp(&b[3])));
        let mut cut = 0;
        let mut e = rectangles[0][3];

        for i in 1..rectangles.len() {
            if rectangles[i][1] < e {
                e = rectangles[i][3].max(e);
            } else if cut == 1 {
                return true;
            } else {
                cut += 1;
                e = rectangles[i][3];
            }
        }

        // vertical
        rectangles.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(a[2].cmp(&b[2])));
        let mut cut = 0;
        let mut e = rectangles[0][2];

        for i in 1..rectangles.len() {
            if rectangles[i][0] < e {
                e = rectangles[i][2].max(e);
            } else if cut == 1 {
                return true;
            } else {
                cut += 1;
                e = rectangles[i][2];
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3394() {
        assert!(Solution::check_valid_cuts(
            5,
            vec![
                vec![1, 0, 5, 2],
                vec![0, 2, 2, 4],
                vec![3, 2, 5, 3],
                vec![0, 4, 4, 5]
            ]
        ));
        assert!(Solution::check_valid_cuts(
            4,
            vec![
                vec![0, 0, 1, 1],
                vec![2, 0, 3, 4],
                vec![0, 2, 2, 3],
                vec![3, 0, 4, 3]
            ]
        ));
        assert!(!Solution::check_valid_cuts(
            4,
            vec![
                vec![0, 2, 2, 4],
                vec![1, 0, 3, 2],
                vec![2, 2, 3, 4],
                vec![3, 0, 4, 2],
                vec![3, 2, 4, 4]
            ]
        ));
    }
}
