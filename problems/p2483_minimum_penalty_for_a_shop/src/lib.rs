pub struct Solution {}

impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let mut right_customers = vec![0; customers.len() + 1];
        for i in (0..customers.len()).rev() {
            if customers.as_bytes()[i] == b'Y' {
                right_customers[i] += 1;
            }
            right_customers[i] += right_customers[i + 1];
        }

        let mut min_penalty = std::i32::MAX;
        let mut min_i = 0;
        let mut penalty = 0;

        for i in 0..customers.len() {
            if penalty + right_customers[i] < min_penalty {
                min_penalty = penalty + right_customers[i];
                min_i = i;
            }
            if customers.as_bytes()[i] == b'N' {
                penalty += 1;
            }
        }
        if penalty < min_penalty {
            customers.len() as i32
        } else {
            min_i as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2483() {
        assert_eq!(Solution::best_closing_time("YYNY".to_string()), 2);
        assert_eq!(Solution::best_closing_time("NNNNN".to_string()), 0);
        assert_eq!(Solution::best_closing_time("YYYY".to_string()), 4);
    }
}
