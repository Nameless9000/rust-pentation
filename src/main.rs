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

fn petration(a: f64, b: f64) -> f64 {
    let mut result: f64 = 1.0;

    for _n in float_loop(0.0, b, 1.0) {
        result = tetration(a, result);
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return println!("Usage: rust-pentation <a: f64> <b: f64>")
    }

    let a = args[1].parse::<f64>().unwrap();
    let b = args[2].parse::<f64>().unwrap();
    

    let output = petration(a, b).to_string();

    println!("{}[5]{} = {}", a, b, output);
}