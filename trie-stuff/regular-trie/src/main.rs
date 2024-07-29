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

}

fn main() {
    let mut cheese = Trie::new();
    println!("{cheese:?}");
}
