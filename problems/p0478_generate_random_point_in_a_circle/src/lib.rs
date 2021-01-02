use rand::{thread_rng, Rng};

pub struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
}

impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            radius,
            x_center,
            y_center,
        }
    }

    fn rand_point(&self) -> Vec<f64> {
        let mut rng = thread_rng();
        let mut x = rng.gen_range(-self.radius, self.radius);
        let mut y = rng.gen_range(-self.radius, self.radius);

        let square_radius = self.radius * self.radius;
        while square_radius < x * x + y * y {
            x = rng.gen_range(-self.radius, self.radius);
            y = rng.gen_range(-self.radius, self.radius);
        }

        vec![x + self.x_center, y + self.y_center]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0487() {
        let obj = Solution::new(1.0, 0.0, 0.0);
        let p1 = obj.rand_point();
        assert!(p1[0] * p1[0] + p1[1] * p1[1] <= 1.0);

        let obj = Solution::new(10.0, 5.0, -7.5);
        let p1 = obj.rand_point();
        assert!((p1[0] - 5.0) * (p1[0] - 5.0) + (p1[1] + 7.5) * (p1[1] + 7.5) <= 100.0);
    }
}
