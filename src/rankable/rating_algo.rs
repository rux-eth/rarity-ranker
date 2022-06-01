pub fn update_rating(rating_f: &f64, score_f: &f64, expected_score_f: &f64) -> f64 {
    let equation = rating_f - (32f64) * (score_f - expected_score_f);
    to_fixed(equation, 2)
}
// Formula 2 Player A (to predict the outcome of a game)
pub fn formula2_pa(rating_a: f64, rating_b: f64) -> f64 {
    let mut equation: f64 = (rating_b - rating_a) / 400f64;
    equation = f64::powf(10f64, equation);

    equation += 1f64;
    equation = 1f64 / equation;
    to_fixed(equation, 2)
}
// Formula 2 Player B(to predict the outcome of a game)
pub fn formula2_pb(rating_a: f64, rating_b: f64) -> f64 {
    let mut equation: f64 = (rating_a - rating_b) / 400f64;
    equation = f64::powf(10f64, equation);
    equation += 1f64;
    equation = 1f64 / equation;
    to_fixed(equation, 2)
}

// Function used to predict outcome
pub fn predict_outcome(ratings: (f64, f64)) -> (f64, f64) {
    let res = (
        formula2_pa(ratings.0, ratings.1),
        formula2_pb(ratings.0, ratings.1),
    );
    println!("{:#?}", res);
    res
}
fn to_fixed(num: f64, precision: i32) -> f64 {
    let d: f64 = (10i32 * precision * precision) as f64;
    (num * d).round() / d
}
