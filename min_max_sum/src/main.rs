/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn mini_max_sum(arr: &[i32]) {
    let total: i64 = arr.iter().map(|&x| x as i64).sum();
    if let (Some(&min), Some(&max)) = (arr.iter().min(), arr.iter().max()) {
        println!("{} {}", total - max as i64, total - min as i64);
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    mini_max_sum(&arr);
}
