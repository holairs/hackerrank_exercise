/*
 * Complete the 'getTotalX' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let min_value = *a.iter().max().unwrap();
    let max_value = *b.iter().min().unwrap();

    (min_value..=max_value)
        .filter(|&i| {
            a.iter().all(|&a_value| i % a_value == 0) && b.iter().all(|&b_value| b_value % i == 0)
        })
        .count() as i32
}

fn main() {
    let a = [2, 4];
    let b = [16, 32, 96];

    let result = get_total_x(&a, &b);
    println!("El valor es: {:?}", result);
}
