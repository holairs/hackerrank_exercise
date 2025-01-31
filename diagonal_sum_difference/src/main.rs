fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut first_diagonal = 0;
    let mut second_diagonal = 0;

    for i in 0..n {
        first_diagonal += arr[i][i];
        second_diagonal += arr[i][n - 1 - i];
    }

    (first_diagonal - second_diagonal).abs()
}

fn main(){
    let arr = vec![vec![11, 2, 4], vec![4, 5, 6], vec![10, 8, -12]];
    let _result = diagonal_difference(&arr);
    let a: i32 = 12;
    let _b = &a;
}
