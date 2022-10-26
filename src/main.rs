use std::env;

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

fn pentation(a: f64, b: f64) -> f64 {
    let mut result: f64 = 1.0;

    for _n in float_loop(0.0, b, 1.0) {
        result = tetration(a, result);
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        return println!("Usage: rust-pentation <type: 1(tetration) | 2(pentation)> <a: f64> <b: f64>")
    }

    let operation = args[1].parse::<i32>().unwrap();

    let a = args[2].parse::<f64>().unwrap();
    let b = args[3].parse::<f64>().unwrap();
    
    let result;

    match operation {
        1=>result = tetration(a, b).to_string(),
        2=>result = pentation(a, b).to_string(),
        _=>return println!("Usage: rust-pentation <type: 1(tetration) | 2(pentation)> <a: f64> <b: f64>"),
    }

    println!("{}[{}]{} = {}", a, 3+operation, b, result);
}