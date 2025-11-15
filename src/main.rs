
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5)); // value is a RefCell containing 5

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil))); // a points to value 5

    let b = Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::clone(&a))); // b points to 3 -> a (5)
    let c = Rc::new(Cons(Rc::new(RefCell::new(4)), Rc::clone(&a))); // c points to 4 -> a (5)

    *value.borrow_mut() += 10; // Modify the value inside the RefCell by adding 10 to it

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}