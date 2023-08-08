use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use std::process;
use std::collections::HashMap;
use csv::ReaderBuilder;

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

fn calculate_range(numbers: &[f64]) -> f64{
    let max_val = numbers.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_val = numbers.iter().cloned().fold(f64::INFINITY, f64::min);
    max_val - min_val
}

fn calculate_variance(numbers: &[f64], mean: f64) -> f64 {
    let mut sum_squared_diff = 0.0;
    for &num in numbers {
        let diff = num - mean;
        sum_squared_diff += diff * diff
    }
    sum_squared_diff / (numbers.len() as f64)
}

fn calculate_coefficient_of_variation(mean:f64, standard_deviation: f64) -> f64 {
    (standard_deviation / mean ) * 100.0
}

fn calculate_skewness(numbers: &[f64], mean:f64, standard_deviation: f64) -> Option<f64> {
    let n = numbers.len() as f64;
    if n < 3.0 || standard_deviation == 0.0 {
        return None;
    }

    let mut sum_cubed_diff = 0.0;
    for &num in numbers {
        let diff = num - mean;
        sum_cubed_diff += diff * diff * diff;
    }
    let skewness = (sum_cubed_diff / (n * standard_deviation.powi(3))).sqrt();
    Some(skewness)
}

fn calculate_mode(numbers: &[f64]) -> Vec<(f64, usize)> {
    
    let mut frequency_map: HashMap<String, usize> = HashMap::new();

    for &num in numbers {
        let num_str = num.to_string();
        *frequency_map.entry(num_str).or_insert(0) += 1;
    }

    let mut max_frequency = 0;
    let mut modes = Vec::new();

    for (value_str, &frequency) in &frequency_map {
        let num = value_str.parse::<f64>().unwrap();
        if frequency > max_frequency {
            max_frequency = frequency;
            modes.clear();
            modes.push((num, frequency));
        } else if frequency == max_frequency {
            modes.push((num, frequency));
        }
    }

    modes
}

fn read_manual_input() -> Result<Vec<f64>, Box<dyn Error>>{
    println!("Enter a list of numbers separated by spaces: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let numbers: Vec<f64> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .collect();

    Ok(numbers)
}

fn read_csv_file(file: File) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

    let mut numbers: Vec<f64> = Vec::new();

    for result in rdr.records(){
        let record = result?;
        for field in record.iter(){
            if let Ok(number) = field.parse::<f64>(){
                numbers.push(number);
            } else {
                eprintln!("Ignoring invalid numbers: {}", field);
            }
        }
    }

    Ok(numbers)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("This program finds the sum, average, and standard deviation of a list of numbers!");
    println!("Enter 'M' for manual input of 'F' for reading from a CSV file: " );

    let mut input_choice = String::new();
    io::stdin().read_line(&mut input_choice)?;

    let numbers: Vec<f64>;

    match input_choice.trim().to_lowercase().as_str(){
        "m" => {
            numbers = read_manual_input()?;
        }
        "f" => {
            println!("Enter the CSV file path: ");
            let mut file_path = String::new();
            io::stdin().read_line(&mut file_path)?;

            let file = File::open(file_path.trim())?;
            numbers = read_csv_file(file)?;
        }
        _ => {
            eprintln!("Invalid choice. Exiting...");
            process::exit(1);
        }
    }  
    
    if numbers.is_empty() {
        println!("No valid numbers found.");
            process::exit(1);
    }
    
    
    let sum = kahan_sum(&numbers);
    let mean = calculate_mean(&numbers);
    let standard_deviation = calculate_standard_deviation(&numbers, mean);
    let median = calculate_median(&numbers);

    let interquartile_range = calculate_interquartile_range(&numbers);
    let q1 = calculate_percentile(&numbers, 25.0);
    let q3 = calculate_percentile(&numbers, 75.0);

    let range = calculate_range(&numbers);
    let variance = calculate_variance(&numbers, mean);
    let coefficient_of_variation = calculate_coefficient_of_variation(mean, standard_deviation);
    let skewness = calculate_skewness(&numbers, mean, standard_deviation);

    let modes = calculate_mode(&numbers);

    println!("Numbers: {:?}", numbers);
    println!("Sum: {:.2}", sum);
    println!("Average: {:.2}", mean);
    println!("Standard Deviation: {:.2}", standard_deviation);
    println!("Median: {:.2}", median);
    println!("25th Percentile (Q1): {:.2}", q1);
    println!("75th Percentile (Q3): {:.2}", q3);
    println!("Interquartile Range: {:.2}", interquartile_range);
    println!("Range: {:.2}", range);
    println!("Variance: {:.2}", variance);
    println!("Coefficient of variation: {:.2} %", coefficient_of_variation);

    match skewness {
        Some(skew) => println!("Skewness: {:.2}", skew),
        None => println!("Skewness: Not enough data to calculate."),
    }
    match modes.len() {
        0 => println!("Modes: No mode found."),
        _ => {
            println!("Mode(s): ");
            for (mode_value, frequency) in modes{
                println!("{:.2} (Frequency: {})", mode_value, frequency);
            }
        }
    }

    Ok(())
}
