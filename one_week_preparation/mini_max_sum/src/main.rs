/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn mini_max_sum(arr: &[i32]) {
    let sum: i64 = arr.iter().map(|&x| x as i64).sum();
    let max: i64 = arr.iter().map(|&x| x as i64).max().unwrap_or(0);
    let min: i64 = arr.iter().map(|&x| x as i64).min().unwrap_or(0);

    println!("{:?} {:?}", sum - max, sum - min)
}

fn main() {
    let arr = [769082435, 210437958, 673982045, 375809214, 380564127];
    mini_max_sum(&arr);
}
