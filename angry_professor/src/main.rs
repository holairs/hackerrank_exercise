/*
 * Complete the 'angryProfessor' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. INTEGER k
 *  2. INTEGER_ARRAY a
 */

fn angry_professor(k: i32, a: &[i32]) -> String {
    let on_time_students = a.iter().filter(|&x| *x <= 0).count() as i32;
    if on_time_students >= k {
        "NO".to_string()
    } else {
        "YES".to_string()
    }
}

fn main() {

}
