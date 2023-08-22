pub struct Solution {}

impl Solution {
    pub fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        let mut remains = vec![0; recipes.len()];
        for i in 0..ingredients.len() {
            remains[i] += ingredients[i].len();
        }

        let mut result = Vec::new();
        let mut supplies = supplies;

        while let Some(supply) = supplies.pop() {
            for i in 0..ingredients.len() {
                if ingredients[i].contains(&supply) {
                    remains[i] -= 1;
                    if remains[i] == 0 {
                        result.push(recipes[i].clone());
                        supplies.push(recipes[i].clone());
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2115() {
        assert_eq!(
            Solution::find_all_recipes(
                vec!["bread".to_string()],
                vec![vec!["yeast".to_string(), "flour".to_string()]],
                vec!["yeast".to_string(), "flour".to_string(), "corn".to_string()]
            ),
            vec!["bread".to_string()]
        );
        assert_eq!(
            Solution::find_all_recipes(
                vec!["bread".to_string(), "sandwich".to_string()],
                vec![
                    vec!["yeast".to_string(), "flour".to_string()],
                    vec!["bread".to_string(), "meat".to_string()]
                ],
                vec!["yeast".to_string(), "flour".to_string(), "meat".to_string()]
            ),
            vec!["bread".to_string(), "sandwich".to_string()]
        );
        assert_eq!(
            Solution::find_all_recipes(
                vec![
                    "bread".to_string(),
                    "sandwich".to_string(),
                    "burger".to_string()
                ],
                vec![
                    vec!["yeast".to_string(), "flour".to_string()],
                    vec!["bread".to_string(), "meat".to_string()],
                    vec![
                        "sandwich".to_string(),
                        "meat".to_string(),
                        "bread".to_string()
                    ]
                ],
                vec!["yeast".to_string(), "flour".to_string(), "meat".to_string()]
            ),
            vec![
                "bread".to_string(),
                "sandwich".to_string(),
                "burger".to_string()
            ]
        );
    }
}
