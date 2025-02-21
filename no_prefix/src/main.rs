use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_word: false,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: &str) -> bool {
        let mut node = &mut self.root;
        let mut is_prefix = false;

        for c in word.chars() {
            if node.is_word {
                is_prefix = true;
                break;
            }
            node = node.children.entry(c).or_insert(TrieNode::new());
        }

        if !node.children.is_empty() || is_prefix || node.is_word {
            return false;
        }

        node.is_word = true;
        true
    }
}

fn no_prefix(words: &[String]) {
    let mut trie = Trie::new();
    for word in words {
        if !trie.insert(word) {
            println!("BAD SET");
            println!("{}", word);
            return;
        }
    }
    println!("GOOD SET");
}

fn main() {
    let words = vec!["a".to_string(), "a".to_string()];
    no_prefix(&words);
}
