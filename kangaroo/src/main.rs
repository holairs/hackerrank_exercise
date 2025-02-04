/*
 * Complete the 'kangaroo' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. INTEGER x1
 *  2. INTEGER v1
 *  3. INTEGER x2
 *  4. INTEGER v2
 */

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    return if v1 == v2 {
        (x1 == x2)
            .then_some("YES".to_string())
            .unwrap_or("NO".to_string())
    } else {
        ((x2 - x1) % (v1 - v2) == 0 && (x2 - x1) / (v1 - v2) >= 0)
            .then_some("YES".to_string())
            .unwrap_or("NO".to_string())
    };
}

fn main() {
    let x1 = 1571;
    let v1 = 4240;
    let x2 = 9023;
    let v2 = 4234;
    //let x1 = 0;
    //let v1 = 3;
    //let x2 = 4;
    //let v2 = 2;
    kangaroo(x1, v1, x2, v2);
}
