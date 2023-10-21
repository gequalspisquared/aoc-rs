use std::cmp;
use std::fs;

pub fn run(file_path: &str) {
    let ingredient_properties = fs::read_to_string(file_path).expect("Failed to get input!");

    let ingredients = parse_ingredient_properties(&ingredient_properties);
    p1(&ingredients);
    p2(&ingredients);
}

fn p1(ingredients: &Vec<Ingredient>) {
    let mut max_score = 0;
    for sugar in 0..=100i64 {
        for sprinkles in 0..=(100 - (sugar)) {
            for candy in 0..=(100 - (sugar + sprinkles)) {
                let chocolate = 100 - (sugar + sprinkles + candy);
                let quantities = vec![sugar, sprinkles, candy, chocolate];
                let score = compute_score(ingredients, &quantities, false);

                max_score = cmp::max(max_score, score);
            }
        }
    }

    println!("The maximum score is {max_score}");
}

fn p2(ingredients: &Vec<Ingredient>) {
    let mut max_score = 0;
    for sugar in 0..=100i64 {
        for sprinkles in 0..=(100 - (sugar)) {
            for candy in 0..=(100 - (sugar + sprinkles)) {
                let chocolate = 100 - (sugar + sprinkles + candy);
                let quantities = vec![sugar, sprinkles, candy, chocolate];
                let score = compute_score(ingredients, &quantities, true);

                max_score = cmp::max(max_score, score);
            }
        }
    }

    println!("The maximum score with 500 calories is {max_score}");
}

fn compute_score(
    ingredients: &Vec<Ingredient>,
    quantities: &Vec<i64>,
    calories_limited: bool,
) -> i64 {
    let mut property_scores: Vec<i64> = vec![0; 4];
    let mut calories = 0;
    for i in 0..ingredients.len() {
        property_scores[0] += quantities[i] * ingredients[i].capacity;
        property_scores[1] += quantities[i] * ingredients[i].durability;
        property_scores[2] += quantities[i] * ingredients[i].flavor;
        property_scores[3] += quantities[i] * ingredients[i].texture;
        calories += quantities[i] * ingredients[i].calories;
    }

    if calories_limited && calories != 500 {
        return 0;
    }

    for score in &property_scores {
        if *score <= 0 {
            return 0;
        }
    }

    property_scores.iter().product()
}

struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

fn parse_ingredient_properties(ingredient_properties: &str) -> Vec<Ingredient> {
    let mut ingredients: Vec<Ingredient> = Vec::new();
    for properties in ingredient_properties.lines() {
        let words: Vec<&str> = properties.split(" ").collect();
        let capacity = parse_with_comma(words[2]);
        let durability = parse_with_comma(words[4]);
        let flavor = parse_with_comma(words[6]);
        let texture = parse_with_comma(words[8]);
        let calories = words[10].parse::<i64>().unwrap();

        ingredients.push(Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        });
    }

    ingredients
}

fn parse_with_comma(word: &str) -> i64 {
    let mut word = word.to_string();
    word.pop();
    word.parse::<i64>().unwrap()
}
