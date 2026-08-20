use std::collections::{HashSet, VecDeque};

#[derive(Default)]
struct RideSharingSystem {
    riders: VecDeque<i32>,
    drivers: VecDeque<i32>,
    canceled: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RideSharingSystem {
    fn new() -> Self {
        Default::default()
    }

    fn add_rider(&mut self, rider_id: i32) {
        self.canceled.remove(&rider_id);
        self.riders.push_back(rider_id);
    }

    fn add_driver(&mut self, driver_id: i32) {
        self.drivers.push_back(driver_id);
    }

    fn match_driver_with_rider(&mut self) -> Vec<i32> {
        if !self.drivers.is_empty() && !self.riders.is_empty() {
            while let Some(r) = self.riders.pop_front() {
                if !self.canceled.remove(&r) {
                    return vec![self.drivers.pop_front().unwrap(), r];
                }
            }
        }
        vec![-1, -1]
    }

    fn cancel_rider(&mut self, rider_id: i32) {
        self.canceled.insert(rider_id);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3829() {
        let mut obj = RideSharingSystem::new();
        obj.add_rider(3);
        obj.add_driver(2);
        obj.add_rider(1);
        assert_eq!(obj.match_driver_with_rider(), vec![2, 3]);
        obj.add_driver(5);
        obj.cancel_rider(3);
        assert_eq!(obj.match_driver_with_rider(), vec![5, 1]);
        assert_eq!(obj.match_driver_with_rider(), vec![-1, -1]);
    }
}

fn main() {}
