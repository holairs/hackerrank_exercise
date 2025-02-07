/*
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let total = arr.len();
    let mut first_diagonal = 0;
    let mut second_diagonal = 0;

    for i in 0..total {
        first_diagonal += arr[i][i];
        second_diagonal += arr[i][total - 1 - i]
    }

    (first_diagonal - second_diagonal).abs()
}

fn main() {
    let arr = vec![vec![11, 2, 4], vec![4, 5, 6], vec![10, 8, -12]];
    let result = diagonal_difference(&arr);
    println!("{}", result)
}
