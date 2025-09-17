use std::collections::{HashMap, BTreeSet};
use std::cmp::Reverse;

struct FoodRatings {
    food_to_cuisine: HashMap<String, String>,
    food_to_rating: HashMap<String, i32>,
    cuisine_to_foods: HashMap<String, BTreeSet<(Reverse<i32>, String)>>,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut food_to_cuisine = HashMap::new();
        let mut food_to_rating = HashMap::new();
        let mut cuisine_to_foods = HashMap::new();

        for i in 0..foods.len() {
            food_to_cuisine.insert(foods[i].clone(), cuisines[i].clone());
            food_to_rating.insert(foods[i].clone(), ratings[i].clone());

            cuisine_to_foods
                .entry(cuisines[i].clone())
                .or_insert_with(BTreeSet::new)
                .insert((Reverse(ratings[i].clone()), foods[i].clone()));
        }

        Self {
            food_to_cuisine,
            food_to_rating,
            cuisine_to_foods,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let cuisine = self.food_to_cuisine.get(&food).unwrap().clone();
        let old_rating = *self.food_to_rating.get(&food).unwrap();

        if let Some(set) = self.cuisine_to_foods.get_mut(&cuisine) {
            set.remove(&(Reverse(old_rating), food.clone()));
            set.insert((Reverse(new_rating), food.clone()));
        }

        self.food_to_rating.insert(food, new_rating);
    }

    fn highest_rated(&self, cuisine: String) -> String {
        let set = self.cuisine_to_foods.get(&cuisine).unwrap();
        set.iter().next().unwrap().1.clone()
    }
}
