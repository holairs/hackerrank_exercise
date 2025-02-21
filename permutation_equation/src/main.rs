#[allow(non_snake_case)]
fn permutationEquation(p: &[i32]) -> Vec<i32> {
    let mut y_result: Vec<i32> = Vec::new();
    let mut list = p.to_vec();
    list.sort();

    for y in 0..p.len() {
        let index = p.iter().position(|&x| x - 1 == y as i32).unwrap();
        y_result.push(index as i32  + 1 - y as i32);
    }

    y_result
}

fn main() {
    let p = vec![2, 3, 1];
    let result = permutationEquation(&p);
    println!("{:?}", result);
}
