use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new("Hello, world!");
    let mut handles = vec![];
    for _ in 0..20 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("Data in thread: {}", data_clone);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
