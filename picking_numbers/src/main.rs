/*
 * Complete the 'pickingNumbers' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY a as parameter.
 */

fn pickingNumbers(a: &[i32]) -> i32 {
    let mut counts = vec![0; 100];
    println!("{:?}", counts);

    for &x in a {
        counts[x as usize] += 1;
    }

    let mut max_length = 0;

    for i in 0..99 {
        let current_length = counts[i] + counts[i + 1];
        max_length = std::cmp::max(max_length, current_length);
    }

    max_length
}

fn main() {
    let a = [1, 1, 2, 2, 4, 4, 5, 5, 5];
    let result = pickingNumbers(&a);
    println!("{:?}", result);
}
