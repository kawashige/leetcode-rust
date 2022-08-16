struct ParkingSystem {
    size: [i32; 3],
    occupied: [i32; 3],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            size: [big, medium, small],
            occupied: [0; 3],
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        if self.occupied[car_type as usize - 1] < self.size[car_type as usize - 1] {
            self.occupied[car_type as usize - 1] += 1;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1603() {
        let mut obj = ParkingSystem::new(1, 1, 0);
        assert!(obj.add_car(1));
        assert!(obj.add_car(2));
        assert!(!obj.add_car(3));
        assert!(!obj.add_car(1));
    }
}
