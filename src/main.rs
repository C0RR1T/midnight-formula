use std::io;
use std::panic::resume_unwind;

fn main() {
    let std = io::stdin();
    loop {
        let mut inputs = String::new();
        let mut amount_string: String = String::new();
        println!("a, b, c");
        std.read_line(&mut inputs).unwrap();
        let mut iter = inputs.split(",")
            .map(|s| s.trim())
            .map(|s| s.parse().unwrap_or_else(|a| {
                println!("Not a number: {}", a);
                std::process::exit(1);
            }));
        let a = iter.next().unwrap_or_else(illegal_input);
        let b = iter.next().unwrap_or_else(illegal_input);
        let c = iter.next().unwrap_or_else(illegal_input);
        println!("Forbidden numbers");
        std.read_line(&mut amount_string).unwrap();
        let amount: Vec<f64> = if amount_string.trim().len() > 0 {
            amount_string.split(",")
                .map(|s| s.trim().parse().unwrap())
                .collect()
        } else { Vec::new() };


        match quadratic_formula(a, b, c) {
            None => println!("no solution"),
            Some((a, b)) => {
                let mut result_string = String::from("L = {");
                if !amount.contains(&a) {
                    result_string.push_str((&a.to_string()));
                }
                if amount.contains(&b) {
                    let mut amountb_string = String::from(",");
                    amountb_string.push_str(&b.to_string());
                    result_string.push_str(&amountb_string);
                }
                result_string.push_str("}");
                println!("{}", result_string);
            }
        }
    }
}

fn quadratic_formula(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let denominator = (2.0 * a) != 0.0;
    let sqr_root = (b.powf(2.0) - (4.0 * a * c)) >= 0.0;
    if sqr_root && denominator {
        Some((
            ((-b + (b.powf(2.0) - (4.0 * a * c)).sqrt()) / (2.0 * a)),
            ((-b - (b.powf(2.0) - (4.0 * a * c)).sqrt()) / (2.0 * a))
        ))
    } else {
        None
    }
}

fn illegal_input<T>() -> T {
    println!("Error! Insert 3 numbers split by a \",\"");
    std::process::exit(1);
}
