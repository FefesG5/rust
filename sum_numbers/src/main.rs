use std::io;

fn kahan_sum(numbers: &[f64]) -> f64 {
    let mut sum = 0.0;
    let mut c = 0.0;
    for &num in numbers{
        let y = num - c;
        let t = sum + y;
        c = (t - sum) - y;
        sum = t;
    }
    sum
}

fn main() {
    println!("This program finds the sum and average of a list of numbers!");
    println!("Enter a list of numbers separated by spaces: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<f64> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .collect();
    
    let sum = kahan_sum(&numbers);
    let average = sum / (numbers.len() as f64);

    println!("Numbers: {:?}", numbers);
    println!("Sum: {:.2}", sum);
    println!("Average {:.2}", average);
}
