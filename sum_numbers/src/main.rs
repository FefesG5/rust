fn main() {
    let array = [10, 20, 30, 40, 50, 60, 70];
    let mut sum:i64 = 0;
    println!("This program finds the sum of the array!");

    for i in 0..array.len(){
        sum+= array[i];
    }
    println!("The sum of the array of numbers {}", sum);
}
