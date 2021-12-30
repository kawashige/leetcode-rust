use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn corp_flight_bookings(mut bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        bookings.sort_unstable_by_key(|booking| booking[0]);

        let mut seats = Vec::with_capacity(n as usize);
        let mut seat = 0;
        let mut i = 0;
        let mut heap = BinaryHeap::new();
        for j in 1..=n {
            while i < bookings.len() && bookings[i][0] <= j {
                seat += bookings[i][2];
                heap.push((Reverse(bookings[i][1]), bookings[i][2]));
                i += 1;
            }
            while let Some((Reverse(last), num)) = heap.peek() {
                if last < &j {
                    seat -= num;
                    heap.pop();
                } else {
                    break;
                }
            }
            seats.push(seat);
        }
        seats
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1109() {
        assert_eq!(
            Solution::corp_flight_bookings(vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]], 5),
            vec![10, 55, 45, 25, 25]
        );
        assert_eq!(
            Solution::corp_flight_bookings(vec![vec![1, 2, 10], vec![2, 2, 15]], 2),
            vec![10, 25]
        );
    }
}
