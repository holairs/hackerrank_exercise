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
    let numbers: [i64; 3] = [1000000001, 1000000002, 1000000005];
    let result = a_very_big_sum(&numbers);
    println!("Suma total: {}", result);
}
