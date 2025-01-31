fn compare_triplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let (temp1, temp2) = a.iter()
        .zip(b.iter())
        .fold((0, 0), |(t1, t2), (&x, &y)| {
            (t1 + (x > y) as i32, t2 + (x < y) as i32)
        });

    vec![temp1, temp2]
}

fn main() {
    println!("{:?}", compare_triplets(&[1, 2, 3], &[3, 2, 1]))
}
