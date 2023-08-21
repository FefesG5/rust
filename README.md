"# rust"

This repository contains different versions of the Rust Calculator, each offering specific features and functionalities.

# Rust Statistical Calculator

## Version 1.0: Command Line Input Rust Calculator - Basic-Input-Value

### Description

This is a command-line statistical calculator. The program allows you to perform various statistical calculations on a list of numbers provided by the user. It calculates the sum, average, standard deviation, median, percentiles, interquartile range, range, variance, coefficient of variation, and skewness of the data.

## Features

- **Input Validation**: The program validates user input to ensure it consists of valid numbers separated by spaces.

- **Accurate Results**: The Kahan summation algorithm is used to improve precision when calculating the sum of numbers, reducing the impact of floating-point errors.

## How to Use

1. **Clone the repository** or download the source code. To clone and use the command line input calculator:

   ```sh
   git clone https://github.com/FefesG5/rust.git --branch Basic-Input-Value --single-branch
   cd rust/sum_numbers
   cargo run --release

   ```

2. **Install Rust**: Make sure you have Rust installed on your system. You can download and install it from the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

3. **Build and Run the Project**: Follow these steps to compile and run the program:

   a. **Open Terminal or Command Prompt**: Start by opening the terminal (on Linux or macOS) or the command prompt (on Windows). This is the command-line interface where you'll enter commands to interact with your computer.

   b. **Navigate to the Project Directory**: Use the `cd` (change directory) command to navigate to the directory where you have the Rust Statistical Calculator source code. For example, if you've downloaded the code to your "Documents" folder, you might navigate to that folder using the `cd` command:

   ```bash
   cd Documents/rust-statistical-calculator
   ```

   Replace "Documents/rust-statistical-calculator" with the actual path to the project directory on your system.

   c. **Build and Run the Project**: Once you're in the project directory, you can use the following command to build and run the program. The `--release` flag is used to build the program with optimizations for release (production) mode, which makes it faster:

   ```bash
   cargo run --release
   ```

   This command tells Cargo (the Rust package manager) to compile the code and execute the resulting binary. After running this command, you should see the program's output in the terminal, which will display the calculated statistical measures based on the input you provide.

4. **Input Numbers**: This program finds the sum, average, and standard deviation of a list of numbers!
   
   Example Input:

   ```plaintext
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

## Version 2.0: Command Line Input with CSV and Manual Input

### Description

The second version of the Rust Calculator is an enhanced command-line calculator that allows you to input numbers manually or from a CSV file.

### Features

- **Manual and CSV Input**: The program supports both manual input of numbers and reading from a CSV file.
- **Input Validation**: User input is validated to ensure it consists of valid numbers.

...

### How to Use

1. **Clone the repository** or download the source code. To clone and use the command line calculator with CSV and manual input options:
   ```sh
   git clone https://github.com/FefesG5/rust.git --branch CSV-Manual-Input --single-branch
   cd rust/calculator_version2
   cargo run --release
   ```

Input Numbers: The program will prompt you to enter a list of numbers separated by spaces. For example:

This program finds the sum, average, and standard deviation of a list of numbers!
Enter 'M' for manual input or 'F' for reading from a CSV file.
Enter Your Choice:

'M' to manually input the numbers by yourself using spaces between the numbers.
'F' to read from a CSV file. Provide the file path (e.g., C:\Users\Downloads\testfile.csv).

## Contributing

If you find any issues or have suggestions for improvements, feel free to open an issue or submit a pull request on GitHub. Your contributions are welcome!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
