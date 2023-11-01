pub struct Solution {}

impl Solution {
    pub fn maximum_bob_points(num_arrows: i32, alice_arrows: Vec<i32>) -> Vec<i32> {
        let mut max_points = 0;
        let mut max_shoots = vec![0; 12];

        for i in 0..2_usize.pow(12) {
            let mut points = 0;
            let mut arrows = 0;
            let mut shoots = vec![0; 12];
            for j in 0..12 {
                if i & 1 << j != 0 {
                    arrows += alice_arrows[j] + 1;
                    points += j;
                    shoots[j] = alice_arrows[j] + 1;
                }
            }

            if arrows <= num_arrows && max_points < points {
                if arrows < num_arrows {
                    shoots[0] += num_arrows - arrows;
                }
                max_shoots = shoots;
                max_points = points;
            }
        }

        max_shoots
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2212() {
        assert_eq!(
            Solution::maximum_bob_points(9, vec![1, 1, 0, 1, 0, 0, 2, 1, 0, 1, 2, 0]),
            vec![0, 0, 0, 0, 1, 1, 0, 0, 1, 2, 3, 1]
        );
        assert_eq!(
            Solution::maximum_bob_points(3, vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2]),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0]
        );
    }
}
