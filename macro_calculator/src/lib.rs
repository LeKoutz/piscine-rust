pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64
}


pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut kcals_sum = 0.0;
    let mut carbs_sum = 0.0;
    let mut proteins_sum = 0.0;
    let mut fats_sum = 0.0;
    for food in foods {
        let kcal: f64 = food.calories.1.trim_end_matches("kcal").parse().unwrap();
        kcals_sum += kcal * food.nbr_of_portions;
        carbs_sum += food.carbs * food.nbr_of_portions;
        proteins_sum += food.proteins * food.nbr_of_portions;
        fats_sum += food.fats * food.nbr_of_portions;
    }
    json::object! {
    cals: (kcals_sum * 100.0).round() / 100.0,
    carbs: (carbs_sum * 100.0).round() / 100.0,
    proteins: (proteins_sum * 100.0).round() / 100.0,
    fats: (fats_sum * 100.0).round() / 100.0,
    }
}
