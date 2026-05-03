use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn min_arrivals_to_discard(arrivals: Vec<i32>, w: i32, m: i32) -> i32 {
        let w = w as usize;
        let mut queue = VecDeque::new();
        let mut count = vec![0; 100_001];
        let mut discards = 0;

        for i in 0..arrivals.len() {
            if w < queue.len() + 1 {
                if let Some(item) = queue.pop_front() {
                    if 0 < item {
                        count[item] -= 1;
                    }
                }
            }
            println!(
                "i: {}, arrivals[i]: {}, queue: {:?}, count: {:?}",
                i,
                arrivals[i],
                queue,
                count[..10].to_vec()
            );
            if m <= count[arrivals[i] as usize] {
                discards += 1;
                queue.push_back(0);
            } else {
                count[arrivals[i] as usize] += 1;
                queue.push_back(arrivals[i] as usize);
            }
        }

        discards
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3679() {
        assert_eq!(
            Solution::min_arrivals_to_discard(vec![1, 2, 1, 3, 1], 4, 2),
            0
        );
        assert_eq!(
            Solution::min_arrivals_to_discard(vec![1, 2, 3, 3, 3, 4], 3, 2),
            1
        );
    }
}
