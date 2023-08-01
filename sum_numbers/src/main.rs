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

fn calculate_percentile(numbers: &[f64], percentile:f64) -> f64 {
    let mut sorted_numbers = numbers.to_vec();
    sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = sorted_numbers.len();
    let index = ({percentile/100.0} * (n as f64 - 1.0)) as usize;
    
    if index >= n - 1 {
        sorted_numbers[n - 1]
    } else {
        let lower_value = sorted_numbers[index];
        let upper_value = sorted_numbers[index + 1];
        lower_value + (percentile / 100.0) * (upper_value - lower_value)
    }
}

fn calculate_interquartile_range(numbers: &[f64]) -> f64 {
    let q3 = calculate_percentile(numbers, 75.0);
    let q1 = calculate_percentile(numbers, 25.0);
    q3 - q1
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

    let interquartile_range = calculate_interquartile_range(&numbers);
    let q1 = calculate_percentile(&numbers, 25.0);
    let q3 = calculate_percentile(&numbers, 75.0);


    println!("Numbers: {:?}", numbers);
    println!("Sum: {:.2}", sum);
    println!("Average {:.2}", mean);
    println!("Standard Deviation {:.2}", standard_deviation);
    println!("Median {:.2}", median);
    println!("25th Percentile (Q1) {:.2}", q1);
    println!("75th Percentile (Q3) {:.2}", q3);
    println!("Interquartile Range {:.2}", interquartile_range);

}
