mod calculations;

use calculations::{
    kahan_sum,
    calculate_mean,
    calculate_standard_deviation,
    calculate_median
};

#[cfg(test)]
mod tests {
    use super::*; 

    fn common_numbers() -> [f64; 5] {
        [1.0, 2.0, 3.0, 4.0, 5.1]
    }

    fn common_negative_numbers() -> [f64; 5]{
        [-1.0, -2.0, -3.0, -4.0, -5.1]
    }

    #[test]
    fn test_kahan_calculations_standard(){
        let numbers = common_numbers();
        assert_eq!(kahan_sum(&numbers), 15.1);
    }

    #[test]
    fn test_kahan_calculations_large_numbers(){
        let numbers = [1e15, 1e15, 1e15];
        assert_eq!(kahan_sum(&numbers), 3e15);
    }

    #[test]
    fn test_kahan_calculations_precision_loss(){
        let numbers = [1.0e10, 1.0, -1.0e10];
        assert_eq!(kahan_sum(&numbers), 1.0);
    }

    #[test]
    fn test_mean_calculations_standard(){
        let numbers = common_numbers();
        let expected_mean = (1.0 + 2.0 + 3.0 + 4.0 + 5.1) / 5.0;
        let calculated_mean = calculate_mean(&numbers);

        assert_eq!(calculated_mean, expected_mean);
    }

    #[test]
    fn test_mean_calculations_large_numbers(){
        let numbers = [1e15, 1e15, 1e15];
        let expected_mean = (1e15 + 1e15 + 1e15) / 3.0;
        let calculated_mean = calculate_mean(&numbers);

        assert_eq!(calculated_mean, expected_mean);
    }

    #[test]
    fn test_mean_calculations_loss_precision(){
        let numbers = [1.0e10, 1.0, -1.0e10];
        let expected_mean = (1.0e10 + 1.0 + -1.0e10) / 3.0;
        let calculated_mean = calculate_mean(&numbers);

        assert_eq!(calculated_mean, expected_mean);
    }

    #[test]
    fn test_standard_deviation_calculations(){
        let numbers = common_numbers();
        let mean = calculate_mean(&numbers);
       
        let calculated_standard_deviation = calculate_standard_deviation(&numbers, mean);

       assert_eq!(calculated_standard_deviation, calculate_expected_standard_deviation())
    }

    fn calculate_expected_standard_deviation() ->f64{
        let numbers = common_numbers();
        let expected_mean = (1.0 + 2.0 + 3.0 + 4.0 + 5.1) / 5.0;

        let mut expected_squared_difference_sum = 0.0;
        for &x in numbers.iter() {
            let difference = x - expected_mean;
            let squared_difference = difference * difference;
            expected_squared_difference_sum += squared_difference;
        }
        
        let mean_of_squared_diff = expected_squared_difference_sum / numbers.len() as f64;
        let expected_standard_deviation = mean_of_squared_diff.sqrt();

        expected_standard_deviation
    }

    #[test]
    fn test_standard_deviation_calculations_negative_numbers(){
        let negative_numbers = common_negative_numbers();
        let mean = calculate_mean(&negative_numbers);
       
        let calculated_standard_deviation = calculate_standard_deviation(&negative_numbers, mean);

        let expected_standard_deviation = calculate_expected_standard_deviation();

        // Allow for small margin of error due to floating-point precision 
       let epsilon = 1e-10;
       assert!((calculated_standard_deviation - expected_standard_deviation).abs() < epsilon);
    }

    fn calculate_expected_standard_deviation_negative_numbers() ->f64{
        let negative_numbers = common_negative_numbers();
        let expected_mean = (-1.0 + -2.0 + -3.0 + -4.0 + -5.1) / 5.0;

        let mut expected_squared_difference_sum = 0.0;
        for &x in negative_numbers.iter() {
            let difference = x - expected_mean;
            let squared_difference = difference * difference;
            expected_squared_difference_sum += squared_difference;
        }
        
        let mean_of_squared_diff = expected_squared_difference_sum / negative_numbers.len() as f64;
        let expected_standard_deviation = mean_of_squared_diff.sqrt();

        expected_standard_deviation
    }

    #[test]
    fn test_median_calculations_odd_numbers_of_elements(){
        let numbers = common_numbers();
        let calculated_median = calculate_median(&numbers);

        assert_eq!(calculated_median, 3.0);
    }

    #[test]
    fn test_median_calculations_even_numbers_of_elements(){
        let numbers = [1.0, 2.0, 3.0, 4.0, 5.1, 6.0];
        let calculated_median = calculate_median(&numbers);

        assert_eq!(calculated_median, 3.5);
    }
}