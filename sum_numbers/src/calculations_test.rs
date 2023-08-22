mod calculations;

use calculations::{
    calculate_mean
};

#[cfg(test)]
mod tests {
    use super::calculate_mean; 

    #[test]
    fn test_mean_calculations(){
        let numbers = [1.0, 2.0, 3.0, 4.0, 5.1];
        let expected_mean = (1.0 + 2.0 + 3.0 + 4.0 + 5.1) / 5.0;
        let calculated_mean = calculate_mean(&numbers);

        assert_eq!(calculated_mean, expected_mean);
    }

   
}