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

fn calculate_mean(numbers: &[f64]) -> f64 {
    let sum = kahan_sum(numbers);
    sum / (numbers.len() as f64)
}

fn calculate_standard_deviation(numbers: &[f64], mean:f64) -> f64 {
    let mut sum_squared_diff = 0.0;
    for &num in numbers {
        let diff = num - mean;
        sum_squared_diff += diff * diff
    }
    let mean_squared_diff = sum_squared_diff / (numbers.len() as f64);
    mean_squared_diff.sqrt()
}

fn calculate_median(numbers: &[f64]) ->f64 {
    let mut sorted_numbers = numbers.to_vec();
    sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = sorted_numbers.len() / 2;
    if sorted_numbers.len() % 2 == 0 {
        (sorted_numbers[mid - 1] + sorted_numbers[mid]) / 2.0
    } else{
        sorted_numbers[mid]
    }
}

fn main() {
    println!("This program finds the sum, average, and standard deviation of a list of numbers!");
    println!("Enter a list of numbers separated by spaces: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<f64> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .collect();
    
    let sum = kahan_sum(&numbers);
    let mean = calculate_mean(&numbers);
    let standard_deviation = calculate_standard_deviation(&numbers, mean);
    let median = calculate_median(&numbers);


    println!("Numbers: {:?}", numbers);
    println!("Sum: {:.2}", sum);
    println!("Average {:.2}", mean);
    println!("Standard Deviation {:.2}", standard_deviation);
    println!("Median {:.2}", median);
}
