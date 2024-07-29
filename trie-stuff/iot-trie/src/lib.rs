use std::{collections::HashMap, cell::RefCell, rc::Rc};

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone, Debug)]
pub struct IoTDevice {
    pub numerical_id: u64,
    pub address: String,
}

pub struct Node {
    pub key: char,
    next: HashMap<char, Link>,
    pub value: Option<IoTDevice>,
}


pub struct DeviceRegistry {
    pub length: u64,
    root: HashMap<char, Link>,
}

impl DeviceRegistry{
    pub fn add(&mut self, device:IoTDevice) {
        let p = device.address.clone();
        let mut path = p.chars();
        if let Some(start) = path.next() {
            todo!()
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
