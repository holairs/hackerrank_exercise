#[allow(non_snake_case)]
fn findDigits(n: i32) -> i32 {
    let values: Vec<char> = n.to_string().chars().collect();
    let mut result = 0;

    for num in values {
        if let Some(digit) = num.to_digit(10) {
            if digit as i32 != 0 && n % digit as i32 == 0 {
                result += 1;
            }
        }
    }

    result
}

fn main() {
    let n = 10;
    let result = findDigits(n);
    println!("Result: {}", result);
}
