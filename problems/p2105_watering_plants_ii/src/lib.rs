pub struct Solution {}

impl Solution {
    pub fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
        let mut a = capacity_a;
        let mut b = capacity_b;
        let mut a_i = 0;
        let mut b_i = plants.len() - 1;
        let mut count = 0;

        while a_i <= b_i {
            if a_i == b_i {
                if b <= a {
                    if a < plants[a_i] {
                        count += 1;
                    }
                } else {
                    if b < plants[b_i] {
                        count += 1;
                    }
                }
            } else {
                if a < plants[a_i] {
                    count += 1;
                    a = capacity_a;
                }
                a -= plants[a_i];
                if b < plants[b_i] {
                    count += 1;
                    b = capacity_b;
                }
                b -= plants[b_i];
            }
            a_i += 1;
            if 0 < b_i {
                b_i -= 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2105() {
        assert_eq!(Solution::minimum_refill(vec![2, 2, 3, 3], 5, 5), 1);
        assert_eq!(Solution::minimum_refill(vec![2, 2, 3, 3], 3, 4), 2);
        assert_eq!(Solution::minimum_refill(vec![5], 10, 8), 0);
    }
}
