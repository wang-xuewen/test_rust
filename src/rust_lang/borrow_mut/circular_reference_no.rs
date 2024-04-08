use std::rc::{Rc, Weak};

use std::cell::RefCell;

use crate::log_a;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn do_circular_ref_no() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    // log_a!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    log_a!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // 对于 Weak 指针，必须通过调用 upgrade() 方法将其转换为 Rc 指针后才能使用
    // upgrade() 方法的作用是尝试获取 Weak 指针所指向数据的强引用。如果数据仍然存在（即强引用计数不为 0），
    // 则 upgrade() 方法会返回 Some(Rc<T>)
    // log_a!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    log_a!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch),
    );
    log_a!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
