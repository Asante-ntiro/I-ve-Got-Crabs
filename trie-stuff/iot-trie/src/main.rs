use std::collections::HashMap;

pub struct Node {
    pub key: char,
    pub value: Option<Arduino>,
    next: HashMap<char, Node>,
}

pub struct Arduino {
    pub model: String,
    pub amount: u8,

}

pub struct ArduinoRegistry {
    pub length: u64,
    root: HashMap<char, Node>
}

fn main() {
    println!("Hello, world!");
}
