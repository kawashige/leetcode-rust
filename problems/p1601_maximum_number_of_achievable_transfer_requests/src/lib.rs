pub struct Solution {}

impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let mut max_number = 0;

        for i in 0..2_usize.pow(requests.len() as u32) {
            let mut balance = vec![0; n as usize];
            for j in 0..requests.len() {
                if i & 1 << j != 0 {
                    balance[requests[j][0] as usize] -= 1;
                    balance[requests[j][1] as usize] += 1;
                }
            }
            if balance.into_iter().all(|b| b == 0) {
                max_number = max_number.max(i.count_ones() as i32);
            }
        }

        max_number
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1601() {
        assert_eq!(
            Solution::maximum_requests(
                5,
                vec![
                    vec![0, 1],
                    vec![1, 0],
                    vec![0, 1],
                    vec![1, 2],
                    vec![2, 0],
                    vec![3, 4]
                ]
            ),
            5
        );
        assert_eq!(
            Solution::maximum_requests(3, vec![vec![0, 0], vec![1, 2], vec![2, 1]]),
            3
        );
    }
}
