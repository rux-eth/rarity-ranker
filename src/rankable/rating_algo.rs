pub fn update_rating(rating_f: &i64, score_f: &f64, expected_score_f: &f64) -> i64 {
    rating_f - (((80f64) * (score_f - expected_score_f)) as i64)
}
// Formula 2 Player A (to predict the outcome of a game)
pub fn formula2_pa(rating_a: i64, rating_b: i64) -> f64 {
    let mut equation: f64 = ((rating_b - rating_a) as f64) / 400f64;
    equation = f64::powf(10f64, equation);

    equation += 1f64;
    equation = 1f64 / equation;
    to_fixed(equation, 2)
}
// Formula 2 Player B(to predict the outcome of a game)
pub fn formula2_pb(rating_a: i64, rating_b: i64) -> f64 {
    let mut equation: f64 = ((rating_a - rating_b) as f64) / 400f64;
    equation = f64::powf(10f64, equation);
    equation += 1f64;
    equation = 1f64 / equation;
    to_fixed(equation, 2)
}

// Function used to predict outcome
pub fn predict_outcome(ratings: (i64, i64)) -> (f64, f64) {
    (
        formula2_pa(ratings.0, ratings.1),
        formula2_pb(ratings.0, ratings.1),
    )
}
fn to_fixed(num: f64, precision: i32) -> f64 {
    let d: f64 = (10i32 * precision * precision) as f64;
    (num * d).round() / d
}
