use std::usize;

#[allow(non_snake_case)]
fn permutationEquation(p: &[i32]) -> Vec<i32> {
    let mut y_result: Vec<i32> = Vec::new();

    for x in 1..=p.len() {
        for y in 1..=p.len() {
            let value: i32 = p[(p[y - 1] - 1) as usize];
            if value == x as i32 {
                y_result.push(y as i32);
            }
        }
    }

    y_result
}

fn main() {
    let p = vec![2, 3, 1];
    let result = permutationEquation(&p);
    println!("{:?}", result);
}
