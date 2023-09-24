use serde::Deserialize;
use std::collections::HashMap;

pub fn kahan_sum(numbers: &[f64]) -> f64 {
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

pub fn round_to_decimal_places(value: f64, decimal_places: usize) -> f64 {
    let multiplier = 10f64.powi(decimal_places as i32);
    (value * multiplier).round() / multiplier
}

pub fn calculate_mean(numbers: &[f64]) -> f64 {
    let sum = kahan_sum(numbers);
    sum / (numbers.len() as f64)
}

pub fn calculate_sample_standard_deviation(numbers: &[f64], mean: f64) -> f64 {
    let sum_squared_diff: f64 = numbers.iter().map(|&num| {
        let diff = num - mean;
        diff * diff
    }).sum();
    let mean_squared_diff = sum_squared_diff / ((numbers.len() - 1) as f64);
    mean_squared_diff.sqrt()
}

pub fn calculate_population_standard_deviation(numbers: &[f64], mean: f64) -> f64 {
    let mut sum_squared_diff = 0.0;
    for &num in numbers {
        let diff = num - mean;
        sum_squared_diff += diff * diff;
    }
    let mean_squared_diff = sum_squared_diff / (numbers.len() as f64);
    mean_squared_diff.sqrt()
}

pub fn calculate_median(numbers: &mut[f64]) ->f64 {
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        (numbers[mid - 1] + numbers[mid]) / 2.0
    } else{
        numbers[mid]
    }
}

pub fn calculate_percentile(numbers: &[f64], percentile: f64) -> f64 {
    let mut sorted_numbers = numbers.to_vec();
    sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = sorted_numbers.len();
    let real_index = percentile / 100.0 * (n as f64 - 1.0);
    let index = real_index as usize;

    if index >= n - 1 {
        sorted_numbers[n - 1]
    } else if real_index.fract() == 0.0 {
        sorted_numbers[index]
    } else {
        let fraction = real_index - index as f64;
        let lower_value = sorted_numbers[index];
        let upper_value = sorted_numbers[index + 1];
        lower_value + fraction * (upper_value - lower_value)
    }
}


pub fn calculate_interquartile_range(numbers: &[f64]) -> f64 {
    let q3 = calculate_percentile(numbers, 75.0);
    let q1 = calculate_percentile(numbers, 25.0);

    println!("Q1: {}", q1);
    println!("Q3: {}", q3);

    q3 - q1
}

pub fn calculate_range(numbers: &[f64]) -> f64{
    let max_val = numbers.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_val = numbers.iter().cloned().fold(f64::INFINITY, f64::min);
    max_val - min_val
}

pub fn calculate_population_variance(numbers: &[f64], mean: f64) -> f64 {
    let mut sum_squared_diff = 0.0;
    for &num in numbers {
        let diff = num - mean;
        sum_squared_diff += diff * diff
    }
    sum_squared_diff / (numbers.len() as f64)
}

pub fn calculate_sample_variance(numbers: &[f64], mean: f64) -> f64 {
    let mut sum_squared_diff = 0.0;
    for &num in numbers {
        let diff = num - mean;
        sum_squared_diff += diff * diff
    }
    sum_squared_diff / (numbers.len() as f64 - 1.0)
}

pub fn calculate_coefficient_of_variation(mean:f64, standard_deviation: f64) -> f64 {
    (standard_deviation / mean ) * 100.0
}

pub fn calculate_sample_skewness(numbers: &[f64], mean: f64, standard_deviation: f64) -> Option<f64> {
    let n = numbers.len() as f64;
    if n < 3.0 || standard_deviation == 0.0 {
        return None;
    }

    let sum_cubed_diff: f64 = numbers.iter().map(|&num| {
        let diff = num - mean;
        diff.powi(3)
    }).sum();

    let adjusted_skewness = (n * (n - 1.0).sqrt() / (n - 2.0)) * (sum_cubed_diff / standard_deviation.powi(3));

    Some(adjusted_skewness)
}


pub fn calculate_population_skewness(numbers: &[f64], mean:f64, standard_deviation: f64) -> Option<f64> {
    let n = numbers.len() as f64;
    if n < 3.0 || standard_deviation == 0.0 {
        return None;
    }

    let mut sum_cubed_diff = 0.0;
    for &num in numbers {
        let diff = num - mean;
        sum_cubed_diff += diff.powi(3);
    }

    let skewness = sum_cubed_diff / (n * standard_deviation.powi(3));  // Corrected the formula here
    Some(skewness)
}