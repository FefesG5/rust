use serde::Deserialize;

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