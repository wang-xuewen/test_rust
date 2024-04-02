use std::cell::RefCell;
use std::rc::Rc;

#[macro_use]
use crate::log_a;

use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn do_circular_ref() {

    let head_str = "rust_lang::circular_reference::do_circular_ref";

    // a -> b
    // b -> a

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    log_a!("{} a initial rc count = {}", head_str,Rc::strong_count(&a));
    log_a!("{} a next item = {:?}", head_str,a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    log_a!("{} a rc count after b creation = {}", head_str,Rc::strong_count(&a));
    log_a!("{} b initial rc count = {}", head_str,Rc::strong_count(&b));
    log_a!("{} b next item = {:?}", head_str,b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    log_a!("{} b rc count after changing a = {}", head_str,Rc::strong_count(&b));
    log_a!("{} a rc count after changing a = {}", head_str,Rc::strong_count(&a));



    // 取消下面的注释行便可以观察到循环引用；它会造成栈的溢出。
    // log_a!("a next item = {:?}", a.tail());
}
