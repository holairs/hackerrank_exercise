/*
 * Complete the 'squares' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER a
 *  2. INTEGER b
 */

fn squares(a: i32, b: i32) -> i32 {
    let mut count = 0;
    let mut i = 1;
    while i * i <= b {
        if i * i >= a {
            count += 1;
        }
        i += 1;
    }
    count
}

fn main() {
    let a = 29;
    let b = 49;
    let result = squares(a, b);
    println!("{}", result);
}
