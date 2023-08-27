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

pub fn calculate_mean(numbers: &[f64]) -> f64 {
    let sum = kahan_sum(numbers);
    sum / (numbers.len() as f64)
}

pub fn calculate_standard_deviation(numbers: &[f64], mean:f64) -> f64 {
    let mut sum_squared_diff = 0.0;
    for &num in numbers {
        let diff = num - mean;
        sum_squared_diff += diff * diff
    }
    let mean_squared_diff = sum_squared_diff / (numbers.len() as f64);
    mean_squared_diff.sqrt()
}

pub fn calculate_median(numbers: &[f64]) ->f64 {
    let mut sorted_numbers = numbers.to_vec();
    sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = sorted_numbers.len() / 2;
    if sorted_numbers.len() % 2 == 0 {
        (sorted_numbers[mid - 1] + sorted_numbers[mid]) / 2.0
    } else{
        sorted_numbers[mid]
    }
}

pub fn calculate_percentile(numbers: &[f64], percentile:f64) -> f64 {
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

pub fn calculate_interquartile_range(numbers: &[f64]) -> f64 {
    let q3 = calculate_percentile(numbers, 75.0);
    let q1 = calculate_percentile(numbers, 25.0);
    q3 - q1
}

pub fn calculate_range(numbers: &[f64]) -> f64{
    let max_val = numbers.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_val = numbers.iter().cloned().fold(f64::INFINITY, f64::min);
    max_val - min_val
}

pub fn calculate_variance(numbers: &[f64], mean: f64) -> f64 {
    let mut sum_squared_diff = 0.0;
    for &num in numbers {
        let diff = num - mean;
        sum_squared_diff += diff * diff
    }
    sum_squared_diff / (numbers.len() as f64)
}

pub fn calculate_coefficient_of_variation(mean:f64, standard_deviation: f64) -> f64 {
    (standard_deviation / mean ) * 100.0
}

pub fn calculate_skewness(numbers: &[f64], mean:f64, standard_deviation: f64) -> Option<f64> {
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

pub fn calculate_mode(numbers: &[f64]) -> Vec<(f64, usize)> {
    
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