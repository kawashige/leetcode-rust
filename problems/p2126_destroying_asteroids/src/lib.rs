pub struct Solution {}

impl Solution {
    pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
        let mut asteroids = asteroids;
        asteroids.sort_unstable();

        let mut mass = mass as usize;
        for asteroid in asteroids {
            if mass < asteroid as usize {
                return false;
            }
            mass += asteroid as usize;
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2126() {
        assert!(Solution::asteroids_destroyed(10, vec![3, 9, 19, 5, 21]));
        assert!(!Solution::asteroids_destroyed(5, vec![4, 9, 23, 4]));
    }
}
