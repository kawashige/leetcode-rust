pub struct Solution {}

impl Solution {
    pub fn minimum_cost(
        s: String,
        t: String,
        flip_cost: i32,
        swap_cost: i32,
        cross_cost: i32,
    ) -> i64 {
        let mut diff_count = [0; 2];
        for i in 0..s.len() {
            if s.as_bytes()[i] != t.as_bytes()[i] {
                diff_count[(s.as_bytes()[i] - b'0') as usize] += 1;
            }
        }

        let mut result = 0;
        if diff_count[1] < diff_count[0] {
            diff_count.swap(0, 1);
        }
        result += (flip_cost * 2).min(swap_cost) as i64 * diff_count[0];
        result += (flip_cost * 2).min(swap_cost + cross_cost) as i64
            * ((diff_count[1] - diff_count[0]) / 2);
        if (diff_count[1] - diff_count[0]) % 2 == 1 {
            result += flip_cost as i64;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3800() {
        assert_eq!(
            Solution::minimum_cost("01000".to_string(), "10111".to_string(), 10, 2, 2),
            16
        );
        assert_eq!(
            Solution::minimum_cost("001".to_string(), "110".to_string(), 2, 100, 100),
            6
        );
        assert_eq!(
            Solution::minimum_cost("1010".to_string(), "1010".to_string(), 5, 5, 5),
            0
        );
    }
}

fn main() {}
