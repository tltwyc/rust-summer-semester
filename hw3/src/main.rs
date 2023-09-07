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

// Ex2: MyRc --------------------------
use std::ops::Deref;

pub struct MyRcRef<T> {
    obj: T,
    cnt: i32
}

impl<T> MyRcRef<T> {
    pub fn new(t: T) -> Self {
        MyRcRef { obj: t, cnt: 1 }
    }
    fn change_cnt(&mut self, delta: i32) {
        self.cnt += delta;
    }
    pub fn inc(&mut self) {
        self.change_cnt(1);
    }
    pub fn dec(&mut self) {
        self.change_cnt(-1);
    }
    pub fn get_cnt(&self) -> i32{
        self.cnt
    }
}

pub struct MyRc<T> {
    ptr: *mut MyRcRef<T>
}

impl<T> MyRc<T> {
    pub fn new(t: T) -> Self {
        Self { ptr: Box::into_raw(Box::new(MyRcRef::new(t))) }
    }
    pub fn strong_count(&self) -> i32 {
        unsafe {
            (*self.ptr).get_cnt()
        }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        unsafe {
            (*self.ptr).inc();
        }
        MyRc { ptr: self.ptr }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &(*self.ptr).obj
        }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            (*self.ptr).dec();
            if (*self.ptr).get_cnt() == 0 {
                drop(Box::from_raw(self.ptr));
            }
        }
    }
}

impl<T> std::fmt::Display for MyRc<T> 
where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", **self)
    }
}

// Ex3: simple_stack ---------------------
use std::cell::RefCell;

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
    println!("Ex1: hash_map!--------------------");
    let map = hash_map!{
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5
    };
    println!("{:?}", map);

    println!("\nEx2: MyRc --------------------------");
    let five = MyRc::new(5);
    let five1 = five.clone();
    let five2 = MyRc::clone(&five1);
    println!("five1 = {}", five1);
    println!("five2 = {}", five2);
    println!("strong_cnt of *five* = {}", MyRc::strong_count(&five1));

    println!("\nEx3: simple_stack ------------------");
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
