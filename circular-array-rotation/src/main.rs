fn circularArrayRotation(a: &[i32], k: i32, queries: &[i32]) -> Vec<i32> {
    let n = a.len();
    let effective_k = k % n as i32;
    let mut result: Vec<i32> = Vec::new();

    println!("{}", effective_k);
    for &i in queries {
        let original_index = (i - effective_k + n as i32) % n as i32;
        result.push(a[original_index as usize]);
    }

    result
}

fn main() {
    let a = vec![3, 4, 5];
    let k = 2;
    let queries = vec![1, 2];
    let result = circularArrayRotation(&a, k, &queries);
    println!("{:?}", result);
}
