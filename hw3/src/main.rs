// Hw3: 41/53/54

// Ex1: hash_map!--------------------
#[macro_use]
extern crate std;
use std::collections::HashMap;

macro_rules! hash_map {
    ($($key:expr => $val:expr),*) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}

// Ex2: 

use std::cell::RefCell;

// struct MyRc<T: Sized> {
//     ptr: Box<RefCell<(T, u32)>>
// }

// impl<T> MyRc<T> {
//     pub fn new(t: T) -> Self {
//         MyRc { ptr: Box::new(RefCell::new((t, 1))) }
//     }
// }

// impl<T> Clone for MyRc<T> {
//     fn clone(&self) -> Self {
//         self.ptr.as_mut().get_mut().1 += 1;
//         MyRc { ptr: Box::new(RefCell::) }
//     }
// }

// struct MyRcRef<'r, T> {
//     obj: &'r T,
//     cnt: &'r i32
// }

// impl<'r, T> MyRcRef<'r, T> {
//     pub fn new(t: &'r T) -> Self {
//         MyRcRef { obj: t, cnt: &1 }
//     }
//     fn change_cnt(&mut self, delta: i32) {
//         let res = *self.cnt + delta;
//         self.cnt = res;
//     }
//     pub fn inc(&mut self) {
//         self.change_cnt(1);
//     }
//     pub fn dec(&mut self) {
//         self.change_cnt(-1);
//     }
//     pub fn get_cnt(&self) -> i32 {
//         *self.cnt
//     }
// }

// // impl<'r, T> Drop for MyRcRef<'r, T> {
// //     fn drop(&mut self) {}
// // }

// pub struct MyRc<'r, T> {
//     ptr: RefCell<MyRcRef<'r, T>>
// }

// impl<'r, T> MyRc<'r, T> {
//     pub fn new(t: &'r T) -> Self {
//         MyRc { ptr: RefCell::new(MyRcRef::new(t)) }
//     }
//     pub fn strong_count(&self) -> i32 {
//         self.ptr.borrow().get_cnt()
//     }
// }

// impl<'r, T> Clone for MyRc<'r, T> {
//     fn clone(&self) -> Self {
//         self.ptr.borrow_mut().inc();
//         MyRc { ptr: RefCell::new(MyRcRef { obj: (self.ptr.borrow().obj), cnt: (self.ptr.borrow().cnt) }) }
//     }
// } 

// Ex3: simple_stack ---------------------
// use std::cell::RefCell;

#[derive(Debug)]
struct SimpleStack<T> {
    stack: RefCell<Vec<T>>,
}

impl<T> SimpleStack<T> {
    fn new() -> SimpleStack<T> {
        SimpleStack { stack: RefCell::new(Vec::new()) }
    }
    fn push(&self, value: T) {
        self.stack.borrow_mut().push(value);
    }
    fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }
}

fn main() {
    // Ex1: hash_map!--------------------
    let map = hash_map!{
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5
    };
    println!("{:?}", map);


    // Ex3: simple_stack ------------------
    let stack = SimpleStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Popped: {:?}", stack.pop());
    println!("Popped: {:?}", stack.pop());

    stack.push(4);

    println!("Popped: {:?}", stack.pop());
    println!("Popped: {:?}", stack.pop());
    println!("Popped: {:?}", stack.pop());

}
