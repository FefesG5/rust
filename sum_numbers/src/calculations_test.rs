mod calculations;

use calculations::{
    round_to_decimal_places,
    kahan_sum,
    calculate_mean,
    calculate_sample_standard_deviation,
    calculate_population_standard_deviation,
    calculate_median,
    calculate_percentile,
    calculate_interquartile_range,
    calculate_range,
    calculate_sample_variance,
    calculate_population_variance,
    calculate_coefficient_of_variation,
    calculate_population_skewness,
    calculate_sample_skewness
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


    // ---------------------------------------- //
    // Kahan Tests
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
    // ---------------------------------------- //
    
    // ---------------------------------------- //
    // Rounding Decimal Tests
    #[test]
    fn test_round_to_decimal_places(){
        let number: f64 = 1.23456789;

        assert_eq!(round_to_decimal_places(number, 0), 1.0);
        assert_eq!(round_to_decimal_places(number, 1), 1.2);
        assert_eq!(round_to_decimal_places(number, 2), 1.23);
        assert_eq!(round_to_decimal_places(number, 3), 1.235);
        assert_eq!(round_to_decimal_places(number, 4), 1.2346);
        assert_eq!(round_to_decimal_places(number, 5), 1.23457);
    }
    
    #[test]
    fn test_round_to_negative_decimal_places(){
        let number: f64 = -1.23456789;

        assert_eq!(round_to_decimal_places(number, 0), -1.0);
        assert_eq!(round_to_decimal_places(number, 1), -1.2);
        assert_eq!(round_to_decimal_places(number, 2), -1.23);
        assert_eq!(round_to_decimal_places(number, 3), -1.235);
        assert_eq!(round_to_decimal_places(number, 4), -1.2346);
        assert_eq!(round_to_decimal_places(number, 5), -1.23457);
    }
    // ---------------------------------------- //
    
    // ---------------------------------------- //
    // Mean Tests
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
    // ---------------------------------------- //


    // ---------------------------------------- //
    // Standard Deviation Tests
    #[test]
    fn test_standard_deviation_calculations(){
        let numbers = common_numbers();
        let mean = calculate_mean(&numbers);
        let epsilon = 1e-10;
       
        let calculated_standard_deviation = calculate_sample_standard_deviation(&numbers, mean);

        let expected_standard_deviation = calculate_expected_sample_standard_deviation();

        assert!((calculated_standard_deviation - expected_standard_deviation).abs() < epsilon, 
        "Calculated sample std standard: {}, but expected: {}",
        calculated_standard_deviation,
        expected_standard_deviation);
    }

    fn calculate_expected_sample_standard_deviation() -> f64 {
        let numbers = common_numbers();
        let expected_mean = (1.0 + 2.0 + 3.0 + 4.0 + 5.1) / 5.0;
    
        let mut expected_squared_difference_sum = 0.0;
        for &x in numbers.iter() {
            let difference = x - expected_mean;
            let squared_difference = difference * difference;
            expected_squared_difference_sum += squared_difference;
        }
        
        let mean_of_squared_diff = expected_squared_difference_sum / (numbers.len() - 1) as f64;
        let expected_standard_deviation = mean_of_squared_diff.sqrt();
    
        expected_standard_deviation
    }
    

    #[test]
    fn test_standard_deviation_calculations_negative_numbers(){
        let negative_numbers = common_negative_numbers();
        let mean = calculate_mean(&negative_numbers);
        let epsilon = 1e-10;
       
        let calculated_standard_deviation = calculate_sample_standard_deviation(&negative_numbers, mean);

        let expected_standard_deviation = calculate_expected_standard_deviation_negative_numbers();

       assert!((calculated_standard_deviation - calculate_expected_standard_deviation_negative_numbers()).abs() < epsilon,
       "Calculated sample std for negative numbers: {}, but expected: {}",
       calculated_standard_deviation,
       expected_standard_deviation
        );
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
        
        let mean_of_squared_diff = expected_squared_difference_sum / (negative_numbers.len() - 1) as f64;
        let expected_standard_deviation = mean_of_squared_diff.sqrt();

        expected_standard_deviation
    }
    // ---------------------------------------- //


    // ---------------------------------------- //
    // Median Tests
    #[test]
    fn test_median_calculations_odd_numbers_of_elements(){
        let mut numbers = common_numbers();
        let calculated_median = calculate_median(&mut numbers[..]);

        assert_eq!(calculated_median, 3.0);
    }

    #[test]
    fn test_median_calculations_even_numbers_of_elements(){
        let mut numbers = [1.0, 2.0, 3.0, 4.0, 5.1, 6.0];
        let calculated_median = calculate_median(&mut numbers[..]);

        assert_eq!(calculated_median, 3.5);
    }
    // ---------------------------------------- //

    // ---------------------------------------- //
    // Percentile Tests
    #[test]
    fn test_percentile_25th_calculations(){
        let numbers = common_numbers();

        let calculated_25_percentile = calculate_percentile(&numbers, 25.0);

        assert_eq!(calculated_25_percentile, 2.0);
    }

    #[test]
    fn test_percentile_75th_calculations(){
        let numbers = common_numbers();

        let calculated_75_percentile = calculate_percentile(&numbers, 75.0);

        assert_eq!(calculated_75_percentile, 4.0);
    }

    #[test]
    fn test_percentile_repeated_values_calculations(){
        let numbers = [2.5, 2.5, 2.5, 2.5, 2.5];

        let calculated_50_percentile = calculate_percentile(&numbers, 50.0);

        assert_eq!(calculated_50_percentile, 2.5);
    }

    // ---------------------------------------- //
    // Interquartile Range Tests
    #[test]
    fn test_interquartile_calculations(){
        let numbers = common_numbers();

        let calculated_interquartile_range = calculate_interquartile_range(&numbers);

        let expected_interquartile_range = 2.00;

        assert_eq!(calculated_interquartile_range, expected_interquartile_range)
    }

    #[test]
    fn test_interquartile_range_negative_numbers(){
        let numbers = common_negative_numbers();

        let calculated_negative_interquartile_range = calculate_interquartile_range(&numbers);

        let expected_negative_interquartile_range = 2.0;

        assert_eq!(calculated_negative_interquartile_range, expected_negative_interquartile_range);
    }

    #[test]
    fn test_interquartile_large_set_calculations(){
        let numbers = vec![0.23, 1.45, 2.89, 3.56, 4.12, 5.98, 6.71, 7.64, 8.29, 9.01];

        let calculated_interquartile_range = calculate_interquartile_range(&numbers);

        let expected_interquartile_range = 4.35;

        assert_eq!(calculated_interquartile_range, expected_interquartile_range)
    }
    // ---------------------------------------- //

    // ---------------------------------------- //
    // Range Tests
    #[test]
    fn test_range_standard_numbers(){
        let numbers = common_numbers();

        let calculated_range = calculate_range(&numbers);

        let expected_range = 4.1;

        assert_eq!(calculated_range, expected_range)

    }

    #[test]
    fn test_range_negative_numbers(){
        let numbers = common_negative_numbers();

        let calculated_range = calculate_range(&numbers);

        let expected_range = 4.1;

        assert_eq!(calculated_range, expected_range)
    }
    // ---------------------------------------- //
  
    // ---------------------------------------- //
    // Sample Variance Tests
    #[test]
    fn test_sample_variance_standard_numbers(){
        let numbers = common_numbers();
        let calculated_mean = calculate_mean(&numbers);
        let epsilon = 1e-10;

        let calculated_variance = calculate_sample_variance(&numbers, calculated_mean);

        let expected_variance = 2.602;

        assert!(
            (calculated_variance - expected_variance).abs() < epsilon,
            "Calculated variance: {}, but expected: {}",
            calculated_variance,
            expected_variance,
        );
    }

    #[test]
    fn test_sample_variance_negative_numbers(){
        let numbers = common_negative_numbers();
        let calculated_mean = calculate_mean(&numbers);
        let epsilon = 1e-10;

        let calculated_variance = calculate_sample_variance(&numbers, calculated_mean);

        let expected_variance = 2.602;

        assert!(
            (calculated_variance - expected_variance).abs() < epsilon,
            "Calculated variance: {}, but expected: {}",
            calculated_variance,
            expected_variance,
        );
    }
    // ---------------------------------------- //
  
    // ---------------------------------------- //
    // Population Variance Tests
    #[test]
    fn test_population_variance_standard_numbers(){
        let numbers = common_numbers();
        let calculated_mean = calculate_mean(&numbers);
        let epsilon = 1e-10;

        let calculated_variance = calculate_population_variance(&numbers, calculated_mean);

        let expected_variance = 2.0816;

        assert!(
            (calculated_variance - expected_variance).abs() < epsilon,
            "Calculated variance: {}, but expected: {}",
            calculated_variance,
            expected_variance,
        );
    }

    #[test]
    fn test_population_variance_negative_standard_numbers(){
        let numbers = common_negative_numbers();
        let calculated_mean = calculate_mean(&numbers);
        let epsilon = 1e-10;

        let calculated_variance = calculate_population_variance(&numbers, calculated_mean);

        let expected_variance = 2.0816;

        assert!(
            (calculated_variance - expected_variance).abs() < epsilon,
            "Calculated variance: {}, but expected: {}",
            calculated_variance,
            expected_variance,
        );
    }
  
    // ---------------------------------------- //
    //  Coefficient of Variation Tests

    #[test]
    fn test_calculate_sample_coefficient_of_variation_standard() {
        let numbers = common_numbers();
        let mean = calculate_mean(&numbers);
        let standard_deviation = calculate_sample_standard_deviation(&numbers, mean);

        let calculated_coefficient = calculate_coefficient_of_variation(mean, standard_deviation);

        let expected_coefficient = 53.4129670236;
        let epsilon = 1e-10;

        assert!(
            (calculated_coefficient- expected_coefficient).abs() < epsilon,
            "Calculated coefficient did not match expected value within tolerance: calculated = {}, expected = {}",
            calculated_coefficient,
            expected_coefficient
        );
    }

    #[test]
    fn test_calculate_population_coefficient_of_variation_standard() {
        let numbers = common_numbers();
        let mean = calculate_mean(&numbers);
        let standard_deviation = calculate_population_standard_deviation(&numbers, mean);

        let calculated_coefficient = calculate_coefficient_of_variation(mean, standard_deviation);

        let expected_coefficient = 47.7740100579;
        let epsilon = 1e-10;

        assert!(
            (calculated_coefficient- expected_coefficient).abs() < epsilon,
            "Calculated coefficient did not match expected value within tolerance: calculated = {}, expected = {}",
            calculated_coefficient,
            expected_coefficient
        );
    }
    // ---------------------------------------- //
  
    // ---------------------------------------- //
    //  Skewness Tests

    #[test]
    fn test_population_skewness_standard(){
        let numbers = common_numbers();
        let mean = calculate_mean(&numbers);
        let standard_deviation = calculate_population_standard_deviation(&numbers, mean);  

        let skewness = calculate_population_skewness(&numbers, mean, standard_deviation);

        let expected_skewness = Some(0.042385649089155604);
        let epsilon = 1e-10;

        assert!(
            matches!(skewness, Some(x) if (x - expected_skewness.unwrap()).abs() < epsilon),
            "Population skewness did not match expected value;  calculated skewness {:?}, expected {:?}",
            skewness,
            expected_skewness
        );
    }

    #[test]
    fn test_sample_skewness_standard() {
        let numbers = common_numbers();
        let mean = calculate_mean(&numbers);
        let standard_deviation = calculate_sample_standard_deviation(&numbers, mean);

        let skewness = calculate_sample_skewness(&numbers, mean, standard_deviation);

        let expected_skewness = Some(0.5054783607136213);

        let epsilon = 1e-10;

        assert!(
            matches!(skewness, Some(x) if (x - expected_skewness.unwrap()).abs() < epsilon),
            "Sample skewness did not match expected value; calculated skewness {:?}, expected {:?}",
            skewness,
            expected_skewness
        );
    }
}