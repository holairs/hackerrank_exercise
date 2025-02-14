/*
 * Complete the 'caesarCipher' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. STRING s
 *  2. INTEGER k
 */

fn caesar_cipher(s: &str, k: i32) -> String {
    let abc_l = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];

    let abc_u = vec![
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];

    let phrase: Vec<char> = s.chars().collect();

    let k = k as usize % abc_l.len();

    let new_order_l = [abc_l[k..].to_vec(), abc_l[..k].to_vec()].concat();
    let new_order_u = [abc_u[k..].to_vec(), abc_u[..k].to_vec()].concat();

    let mut result: Vec<String> = Vec::new();

    for letter in phrase {
        let mut found: bool = false;
        for i in 0..26 {
            if let Some(char) = abc_l[i].chars().next() {
                if letter == char {
                    result.push(new_order_l[i].to_string());
                    found = true
                }
            }
            if let Some(char) = abc_u[i].chars().next() {
                if letter == char {
                    result.push(new_order_u[i].to_string());
                    found = true
                }
            }
        }
        if !found {
            result.push(letter.to_string());
        }
    }

    result.join("")
}

fn main() {
    let s = "middle-Outz";
    let k = 2;
    let result = caesar_cipher(s, k);
    println!("{}", result);
}
