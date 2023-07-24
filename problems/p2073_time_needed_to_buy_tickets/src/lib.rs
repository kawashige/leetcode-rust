pub struct Solution {}

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut tickets = tickets;
        let mut t = 0;
        loop {
            for i in 0..tickets.len() {
                if 0 < tickets[i] {
                    t += 1;
                    tickets[i] -= 1;
                    if i == k as usize && tickets[i] == 0 {
                        return t;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2073() {
        assert_eq!(Solution::time_required_to_buy(vec![2, 3, 2], 2), 6);
        assert_eq!(Solution::time_required_to_buy(vec![5, 1, 1, 1], 0), 8);
    }
}
