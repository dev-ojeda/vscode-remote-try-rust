#![allow(unused)]
use std::iter;

// #[derive(Debug)]
// struct Counter {
//     length: usize,
//     count: usize,
// }

// impl Counter {
//     fn new(length: usize) -> Counter {
//         Counter { count: 0, length }
//     }
// }

// impl Iterator for Counter {
//     type Item = usize;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.count += 1;
//         if self.count <= self.length {
//             Some(self.count)
//         } else {
//             None
//         }
//     }
// }

// struct Container {
//     value: u32,
// }

// impl Container {
//     pub fn new(value: u32) -> Self {
//         Container { value }
//     }
// }

// /* EJERCICIO */
#[derive(Debug, PartialEq)]
struct Groups<T> {
    inner: Vec<T>,
}

impl<T> Groups<T> {
    fn new(inner: Vec<T>) -> Self { Groups { inner } }
}

impl<T: PartialEq> Iterator for Groups<T>  {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }

}



fn main() {
    
}
