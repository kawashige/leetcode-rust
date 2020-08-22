pub struct Solution {
    rects: Vec<Vec<i32>>,
    areas: Vec<i32>,
    area_total: i32,
}

use rand::{thread_rng, Rng};

impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut area_total = 0;
        let areas = rects
            .clone()
            .into_iter()
            .map(|v| {
                area_total += (v[2] - v[0] + 1).abs() * (v[3] - v[1] + 1).abs();
                area_total
            })
            .collect::<Vec<i32>>();
        Self {
            rects: rects,
            areas,
            area_total,
        }
    }

    fn pick(&mut self) -> Vec<i32> {
        let mut rng = thread_rng();
        let index = match self
            .areas
            .binary_search(&rng.gen_range(0, self.area_total + 1))
        {
            Ok(i) => i,
            Err(i) => i,
        };

        vec![
            rng.gen_range(self.rects[index][0], self.rects[index][2] + 1),
            rng.gen_range(self.rects[index][1], self.rects[index][3] + 1),
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day22() {
        let mut obj = Solution::new(vec![vec![-2, -2, -1, -1], vec![1, 0, 3, 0]]);
        for _ in 0..4 {
            println!("{:?}", obj.pick());
        }
    }
}
