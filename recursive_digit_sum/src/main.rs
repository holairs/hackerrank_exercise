/*
 * Complete the 'superDigit' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. STRING n
 *  2. INTEGER k
 */

#[allow(non_snake_case)]
fn superDigit(n: &str, k: i32) -> i32 {
    let sum: i128 = n.chars().map(|x| x.to_digit(10).unwrap() as i128).sum();
    let mut num: i128 = sum * (k as i128);

    while num >= 10 {
        num = num
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i128)
            .sum();
    }

    num as i32
}

fn main() {
    let n = "9875";
    let k = 4;
    let result = superDigit(n, k);
    println!("{}", result);
}
