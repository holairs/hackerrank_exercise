/*
 * Complete the 'formingMagicSquare' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY s as parameter.
 */

fn formingMagicSquare(s: &[Vec<i32>]) -> i32 {
    let magic_squares: [[[i32; 3]; 3]; 8] = [
        [[8, 1, 6], [3, 5, 7], [4, 9, 2]],
        [[6, 1, 8], [7, 5, 3], [2, 9, 4]],
        [[4, 9, 2], [3, 5, 7], [8, 1, 6]],
        [[2, 9, 4], [7, 5, 3], [6, 1, 8]],
        [[8, 3, 4], [1, 5, 9], [6, 7, 2]],
        [[4, 3, 8], [9, 5, 1], [2, 7, 6]],
        [[6, 7, 2], [1, 5, 9], [8, 3, 4]],
        [[2, 7, 6], [9, 5, 1], [4, 3, 8]],
    ];

    let mut last_matrix_cost = 100;
    let mut count = 0;

    for square in 0..8 {
        for row in 0..3 {
            for column in 0..3 {
                let actual_value = s[row][column];
                count += (actual_value - magic_squares[square][row][column]).abs()
            }
        }
        if last_matrix_cost >= count {
            last_matrix_cost = count
        }
        count = 0
    }

    last_matrix_cost
}

fn main() {
    let s = [vec![5, 3, 4], vec![1, 5, 8], vec![6, 4, 2]];
    let result = formingMagicSquare(&s);
    println!("{}", result);
}
