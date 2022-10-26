fn float_loop(start: f64, threshold: f64, step_size: f64) -> impl Iterator<Item = f64> {
    std::iter::successors(Some(start), move |&prev| {
        let next = prev + step_size;
        (next < threshold).then_some(next)
    })
}

fn tetration(a: f64, b: f64) -> f64 {
    let mut result: f64 = 1.0;

    for _n in float_loop(0.0, b, 1.0) {
        result = a.powf(result);
    }

    result
}

fn petration(a: f64, b: f64) -> f64 {
    let mut result: f64 = 1.0;

    for _n in float_loop(0.0, b, 1.0) {
        result = tetration(a, result);
    }

    result
}

fn main() {
    let output = petration(2.0, 2.0).to_string();
    println!("Result: {}", output);
}