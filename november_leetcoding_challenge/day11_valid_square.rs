pub struct Solution {}

impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let mut p = vec![p1, p2, p3, p4];
        p.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));

        println!("{:?}", p);

        (p[1][0] - p[0][0]) * (p[2][0] - p[0][0]) + (p[1][1] - p[0][1]) * (p[2][1] - p[0][1]) == 0
            && (p[0][0] - p[1][0]) * (p[3][0] - p[1][0]) + (p[0][1] - p[1][1]) * (p[3][1] - p[1][1])
                == 0
            && !(p[0][0] == p[1][0] && p[0][1] == p[1][1])
            && (((p[1][0] - p[0][0]).pow(2) + (p[1][1] - p[0][1]).pow(2)) as f32).sqrt()
                == (((p[2][0] - p[0][0]).pow(2) + (p[2][1] - p[0][1]).pow(2)) as f32).sqrt()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day11() {
        assert!(Solution::valid_square(
            vec![1, 0],
            vec![-1, 0],
            vec![0, 1],
            vec![0, -1]
        ));

        assert!(Solution::valid_square(
            vec![0, 0],
            vec![1, 1],
            vec![1, 0],
            vec![0, 1]
        ));

        assert!(!Solution::valid_square(
            vec![0, 0],
            vec![1, 0],
            vec![0, 1],
            vec![0, 1]
        ));

        assert!(!Solution::valid_square(
            vec![0, 0],
            vec![1, 2],
            vec![1, 0],
            vec![0, 2]
        ));
    }
}
