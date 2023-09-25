use std::rc::Rc;

struct Secret(u32);

fn main() {
    let data = Rc::new(Secret(1234));
    let data_clone = Rc::clone(&data);
    println!("Original: {}", data);
    println!("Clone: {}", data_clone);
}
