struct DetectSquares {
    points: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DetectSquares {
    fn new() -> Self {
        Self {
            points: vec![vec![0; 1001]; 1001],
        }
    }

    fn add(&mut self, point: Vec<i32>) {
        self.points[point[0] as usize][point[1] as usize] += 1;
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        let mut count = 0;
        let (x, y) = (point[0] as usize, point[1] as usize);
        for y2 in 0..1001 {
            if y == y2 || self.points[x][y2] == 0 {
                continue;
            }
            let d = (point[1] - y2 as i32).abs() as usize;
            if d <= x {
                count += self.points[x][y2] * self.points[x - d][y2] * self.points[x - d][y];
            }
            if d + x <= 1000 {
                count += self.points[x][y2] * self.points[x + d][y2] * self.points[x + d][y];
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2013() {
        let mut obj = DetectSquares::new();
        obj.add(vec![3, 10]);
        obj.add(vec![11, 2]);
        obj.add(vec![3, 2]);
        assert_eq!(obj.count(vec![11, 10]), 1);
        assert_eq!(obj.count(vec![14, 8]), 0);
        obj.add(vec![11, 2]);
        assert_eq!(obj.count(vec![11, 10]), 2);
    }
}
