use std::collections::HashMap;


#[derive(Default, Debug)]
struct Node {
    end_of_word: bool,
    children: HashMap<char, Node>,
}

#[derive(Default, Debug)]
pub struct Trie {
    root: Node,
    len: usize,
}

impl Trie {
    pub fn new() -> Self {
        Trie::default()
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;

        for c in word.chars() {
            current_node = current_node.children
                .entry(c)
                .or_default();
        }

        current_node.end_of_word = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut current_node = &self.root;
    }

}

fn main() {
    let mut cheese = Trie::new();
    println!("{cheese:?}");
}
