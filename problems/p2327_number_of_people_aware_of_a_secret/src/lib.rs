use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        const M: i32 = 1_000_000_007;

        let mut delay_queue = VecDeque::new();
        let mut forget_queue = VecDeque::new();
        delay_queue.push_back((1 + delay, 1));
        forget_queue.push_back((1 + forget, 1));

        let mut known_people = 1;
        let mut sharerable_people = 0;

        for i in 1 + delay..=n {
            while let Some((t, p)) = delay_queue.pop_front() {
                if i < t {
                    delay_queue.push_front((t, p));
                    break;
                }
                sharerable_people += p;
                sharerable_people %= M;
            }
            while let Some((t, p)) = forget_queue.pop_front() {
                if i < t {
                    forget_queue.push_front((t, p));
                    break;
                }
                sharerable_people -= p;
                while sharerable_people < 0 {
                    sharerable_people += M;
                }
                sharerable_people %= M;
                known_people -= p;
                while known_people < 0 {
                    known_people += M;
                }
                known_people %= M;
            }
            delay_queue.push_back((i + delay, sharerable_people));
            forget_queue.push_back((i + forget, sharerable_people));
            known_people += sharerable_people;
            known_people %= M;
        }

        known_people
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2327() {
        assert_eq!(Solution::people_aware_of_secret(1000, 1, 1000), 5);
        assert_eq!(Solution::people_aware_of_secret(6, 2, 4), 5);
        assert_eq!(Solution::people_aware_of_secret(4, 1, 3), 6);
    }
}
