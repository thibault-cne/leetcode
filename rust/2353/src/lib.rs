use std::{
    cmp::Reverse,
    collections::{BTreeSet, HashMap},
};

struct FoodRatings {
    food_to_rating_cuisine: HashMap<String, (i32, String)>,
    cuisine_to_food_rating: HashMap<String, BTreeSet<(i32, Reverse<String>)>>,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut food_to_rating_cuisine = HashMap::new();
        let mut cuisine_to_food_rating: HashMap<String, BTreeSet<(i32, Reverse<String>)>> =
            HashMap::new();
        for ((food, cuisine), rating) in foods.into_iter().zip(cuisines).zip(ratings) {
            food_to_rating_cuisine.insert(food.clone(), (rating, cuisine.clone()));
            cuisine_to_food_rating
                .entry(cuisine)
                .or_default()
                .insert((rating, Reverse(food)));
        }

        Self {
            food_to_rating_cuisine,
            cuisine_to_food_rating,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let (rating, cuisine) = self.food_to_rating_cuisine.get_mut(&food).unwrap();
        let old_rating = std::mem::replace(rating, new_rating);

        let food_rating_set = self.cuisine_to_food_rating.get_mut(cuisine).unwrap();
        food_rating_set.remove(&(old_rating, Reverse(food.clone())));
        food_rating_set.insert((new_rating, Reverse(food)));
    }

    fn highest_rated(&self, cuisine: String) -> String {
        self.cuisine_to_food_rating
            .get(&cuisine)
            .unwrap()
            // the two lines below should've been just `last`, but it isn't stable in Leetcode
            .iter()
            .next_back()
            .unwrap()
            .1
             .0
            .to_owned()
    }
}
