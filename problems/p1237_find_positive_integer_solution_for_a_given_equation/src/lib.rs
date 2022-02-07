// This is the custom function interface.
// You should not implement it, or speculate about its implementation
pub struct CustomFunction;
impl CustomFunction {
    pub fn f(&self, x: i32, y: i32) -> i32 {
        x + y
    }
}

pub struct Solution {}

impl Solution {
    pub fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        for x in 1..1001 {
            for y in 1..1001 {
                let v = customfunction.f(x, y);
                if v == z {
                    result.push(vec![x, y]);
                } else if z < v {
                    break;
                }
            }
        }

        result
    }
}
