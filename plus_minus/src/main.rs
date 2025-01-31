fn plus_minus(arr: &[i32]) {
    let len = arr.len() as f64;
    let positive_values = arr.iter().filter(|&x| *x > 0).count() as f64 / len;
    let zero_values = arr.iter().filter(|&x| *x == 0).count() as f64 / len;
    let negative_values = arr.iter().filter(|&x| *x < 0).count() as f64 / len;
    println!("{:?}", positive_values);
    println!("{:?}", negative_values);
    println!("{:?}", zero_values);

    let a: String = 100.to_string();
    let b = a;
    println!("{}", a);

}

fn main() {
    let arr = [0, 0, -1, 1, 1];
    plus_minus(&arr);
}
