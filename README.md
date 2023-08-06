"# rust" 


# Rust Statistical Calculator

This is a command-line statistical calculator written in Rust. The program allows you to perform various statistical calculations on a list of numbers provided by the user. It calculates the sum, average, standard deviation, median, percentiles, interquartile range, range, variance, coefficient of variation, and skewness of the data.

## Features

- **Input Validation**: The program validates user input to ensure it consists of valid numbers separated by spaces.

- **Accurate Results**: The Kahan summation algorithm is used to improve precision when calculating the sum of numbers, reducing the impact of floating-point errors.

## How to Use

1. **Clone the repository** or download the source code.

2. **Install Rust**: Make sure you have Rust installed on your system. You can download and install it from the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

3. **Build and Run the Project**: Open your terminal or command prompt, navigate to the project directory, and run the following command to build and run the project:

```bash
cargo run --release
```

4. **Input Numbers**: The program will prompt you to enter a list of numbers separated by spaces. For example:

```plaintext
This program finds the sum, average, and standard deviation of a list of numbers!
Enter a list of numbers separated by spaces:
1.5 2.8 3.2 4.1 5.6
```

5. **View Results**: The program will display the calculated statistical measures for the provided numbers:

```plaintext
Numbers: [1.5, 2.8, 3.2, 4.1, 5.6]
Sum: 17.20
Average: 3.44
Standard Deviation: 1.40
Median: 3.20
25th Percentile (Q1): 2.80
75th Percentile (Q3): 4.10
Interquartile Range: 1.30
Range: 4.10
Variance: 1.95
Coefficient of variation: 40.81 %
Skewness: 0.34
```

## Contributing

If you find any issues or have suggestions for improvements, feel free to open an issue or submit a pull request on GitHub. Your contributions are welcome!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
