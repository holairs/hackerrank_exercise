/*
 * Complete the 'utopianTree' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER n as parameter.
 */

fn utopian_tree(n: i32) -> i32 {
    let mut result = 1;
    for i in 1..=n {
        if i % 2 == 1 {
            result *= 2;
        } else {
            result += 1;
        }
    }
    result
}

fn main() {
    let n = 5;
    let result = utopian_tree(n);
    println!("{}", result);
}
