fn main() {
    print!("{}", solution("abcdef", "def"));
}

fn solution(word: &str, ending: &str) -> bool {
    word.ends_with(ending)
}
