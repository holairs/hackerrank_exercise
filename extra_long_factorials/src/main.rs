#![allow(non_snake_case)]

fn extraLongFactorials(n: i32) {
    let mut result = vec![1];
    for i in 1..=n {
        let mut carry = 0;
        for j in 0..result.len() {
            let product = result[j] * i + carry;
            result[j] = product % 10;
            carry = product / 10;
        }
        while carry > 0 {
            result.push(carry % 10);
            carry /= 10;
        }
    }
    result.reverse();
    println!("{}", result.iter().map(|x| x.to_string()).collect::<String>());
}

fn main() {
    //let n = 25;
    // 15511210043330985984000000
    let n = 45;
    // 119622220865480194561963161495657715064383733760000000000
    extraLongFactorials(n)
}
