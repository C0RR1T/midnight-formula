use std::io;

enum Solution {
    NoSolution,
    OneSolution(f64),
    TwoSolutions(f64, f64),
}

fn main() {
    let std = io::stdin();
    loop {
        let mut inputs = String::new();
        let mut amount_string: String = String::new();
        println!("a, b, c");
        std.read_line(&mut inputs).unwrap();
        let mut iter = inputs.split(',').map(|s| s.trim()).map(|s| {
            s.parse().unwrap_or_else(|a| {
                println!("Not a number: {}", a);
                std::process::exit(1);
            })
        });
        let a = iter.next().unwrap_or_else(illegal_input);
        let b = iter.next().unwrap_or_else(illegal_input);
        let c = iter.next().unwrap_or_else(illegal_input);
        println!("Forbidden numbers");
        std.read_line(&mut amount_string).unwrap();
        let forbidden_numbers: Vec<f64> = if !amount_string.trim().is_empty() {
            amount_string
                .split(',')
                .map(|s| {
                    s.trim().parse::<f64>().unwrap_or_else(|a| {
                        println!("Not a number: {}", a);
                        std::process::exit(1);
                    })
                })
                .collect()
        } else {
            Vec::new()
        };

        match quadratic_formula(a, b, c, &forbidden_numbers) {
            Solution::NoSolution => println!("No solution"),
            Solution::OneSolution(x) => println!("One Solution: x1 = {}", x),
            Solution::TwoSolutions(x1, x2) => println!("Two Solutions:\nx1 = {}\nx2 = {} ", x1, x2),
        }
    }
}

fn quadratic_formula(a: f64, b: f64, c: f64, forbidden_numbers: &[f64]) -> Solution {
    let denominator = 2.0 * a;
    if denominator == 0.0 {
        return Solution::NoSolution;
    }
    let sqr_root = b.powf(2.0) - 4.0 * a * c;
    if sqr_root == 0.0 {
        let value = (-b + 0.0) / denominator;
        if forbidden_numbers.contains(&value) {
            Solution::NoSolution
        } else {
            Solution::OneSolution(value)
        }
    } else {
        match sqr_root {
            x if x < 0.0 => Solution::NoSolution,

            x => {
                let first_value = (-b + x.sqrt()) / denominator;
                let second_value = (-b - x.sqrt()) / denominator;
                let contains_first_value = forbidden_numbers.contains(&first_value);
                let contains_second_value = forbidden_numbers.contains(&second_value);

                if contains_first_value && contains_second_value {
                    Solution::NoSolution
                } else if contains_first_value {
                    Solution::OneSolution(second_value)
                } else if contains_second_value {
                    Solution::OneSolution(first_value)
                } else {
                    Solution::TwoSolutions(first_value, second_value)
                }
            }
        }
    }
}

fn illegal_input<T>() -> T {
    println!("Error! Insert 3 numbers split by a \",\"");
    std::process::exit(1);
}
