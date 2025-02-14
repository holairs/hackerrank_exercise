/*
 * Complete the 'isBalanced' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn isBalanced(s: &str) -> String {
    let mut stack: Vec<char> = Vec::new();

    for sym in s.chars() {
        match sym {
            '{' | '(' | '[' => stack.push(sym),
            '}' => {
                if stack.is_empty() || stack.pop().unwrap() != '{' {
                    return "NO".to_string();
                }
            }
            ')' => {
                if stack.is_empty() || stack.pop().unwrap() != '(' {
                    return "NO".to_string();
                }
            }
            ']' => {
                if stack.is_empty() || stack.pop().unwrap() != '[' {
                    return "NO".to_string();
                }
            }
            _ => return "NO".to_string(),
        }
    }

    if stack.is_empty() {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

fn main() {
    let s = "{(([])[])[]}";
    let result = isBalanced(s);
    println!("{}", result); // Imprime YES
    let s2 = "{(([])[])[]]}";
    let result2 = isBalanced(s2);
    println!("{}", result2); // Imprime NO
    let s3 = "{(([])[])[]}[]";
    let result3 = isBalanced(s3);
    println!("{}", result3); // Imprime YES
}

fn main() {
    let s = "{(([])[])[]}";
    //let s = "{(([])[])[]]}";
    //let s = "{(([])[])[]}[]";
    let result = isBalanced(s);
    println!("{}", result);
}
