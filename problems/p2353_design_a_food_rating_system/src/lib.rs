use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

struct FoodRatings {
    cuisine_rating: HashMap<String, BinaryHeap<(i32, Reverse<String>)>>,
    rating: HashMap<String, i32>,
    cuisine: HashMap<String, String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut cuisine_rating = HashMap::new();
        let mut rating = HashMap::new();
        let mut cuisine = HashMap::new();
        for i in 0..foods.len() {
            cuisine_rating
                .entry(cuisines[i].clone())
                .or_insert(BinaryHeap::new())
                .push((ratings[i], Reverse(foods[i].clone())));
            rating.insert(foods[i].clone(), ratings[i]);
            cuisine.insert(foods[i].clone(), cuisines[i].clone());
        }

        Self {
            cuisine_rating,
            rating,
            cuisine,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        self.cuisine_rating
            .get_mut(&self.cuisine[&food])
            .unwrap()
            .push((new_rating, Reverse(food.clone())));
        self.rating.insert(food, new_rating);
    }

    fn highest_rated(&mut self, cuisine: String) -> String {
        let heap = self.cuisine_rating.get_mut(&cuisine).unwrap();
        while let Some((rating, Reverse(food))) = heap.pop() {
            if self.rating[&food] == rating {
                heap.push((rating, Reverse(food.to_string())));
                return food;
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2353() {
        let mut obj = FoodRatings::new(
            vec![
                "kimchi".to_string(),
                "miso".to_string(),
                "sushi".to_string(),
                "moussaka".to_string(),
                "ramen".to_string(),
                "bulgogi".to_string(),
            ],
            vec![
                "korean".to_string(),
                "japanese".to_string(),
                "japanese".to_string(),
                "greek".to_string(),
                "japanese".to_string(),
                "korean".to_string(),
            ],
            vec![9, 12, 8, 15, 14, 7],
        );
        assert_eq!(
            obj.highest_rated("korean".to_string()),
            "kimchi".to_string()
        );
        assert_eq!(
            obj.highest_rated("japanese".to_string()),
            "ramen".to_string()
        );
        obj.change_rating("sushi".to_string(), 16);
        assert_eq!(
            obj.highest_rated("japanese".to_string()),
            "sushi".to_string()
        );
        obj.change_rating("ramen".to_string(), 16);
        assert_eq!(
            obj.highest_rated("japanese".to_string()),
            "ramen".to_string()
        );
    }
}
