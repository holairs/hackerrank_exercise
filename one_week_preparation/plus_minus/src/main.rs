/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plus_minus(arr: &[i32]) {
    let max = arr.iter().count() as f32;
    let mut negative = 0;
    let mut positive = 0;
    let mut zero = 0;

    for num in arr {
        if num < &0 {
            negative += 1;
        }

        if num > &0 {
            positive += 1;
        }

        if num == &0 {
            zero += 1;
        }
    }

    println!("{:.6}", positive as f32 / max);
    println!("{:.6}", negative as f32 / max);
    println!("{:.6}", zero as f32 / max)
}

fn main() {
    let arr = [-4, 3, -9, 0, 4, 1];
    plus_minus(&arr);
}
