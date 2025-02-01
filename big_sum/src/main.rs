/*
 * Complete the 'aVeryBigSum' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts LONG_INTEGER_ARRAY ar as parameter.
 */

fn a_very_big_sum(ar: &[i64]) -> i64 {
    ar.iter().sum()
}

fn main() {
    let numbers: [i64; 3] = [2, 2, 1];
    let result = a_very_big_sum(&numbers);
    println!("Suma total: {}", result);
}
