use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{
    Rc,
    Weak
};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn next_tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct TreeNode {
    value: i32,
    parent: RefCell<Weak<TreeNode>>,
    child: RefCell<Vec<Rc<TreeNode>>>,
}

fn main() {
    println!("test Rc and RefCell.");
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a tail: {:?}", a.next_tail());
    println!("a rc count: {}", Rc::strong_count(&a));
    println!("--------------------------------");
    // b 指向 a
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b init: {}", Rc::strong_count(&a));
    println!("b tail: {:?}", b.next_tail());
    println!("b rc count: {}", Rc::strong_count(&b));

    if let Some(linker) = a.next_tail() {
        *linker.borrow_mut() = Rc::clone(&b);
    }
    println!("--------------------------------");
    println!("a rc count: {}", Rc::strong_count(&a));
    // println!("a tail: {:?}", a.next_tail());
    println!("b rc count: {}", Rc::strong_count(&b));
    // println!("b tail: {:?}", b.next_tail());

    println!("test Weak");

    let leaf = Rc::new(
        TreeNode {
            value: 5,
            parent: RefCell::new(Weak::new()),
            child: RefCell::new(vec![]),
        }
    );

    //  upgrade 返回
    println!("left's parent: {:?}", leaf.parent.borrow_mut().upgrade());

    let root = Rc::new(
        TreeNode {
            value: 10,
            parent: RefCell::new(Weak::new()),
            child: RefCell::new(vec![Rc::clone(&leaf)]),
        }
    );

    // 分配新的 weak 指针
    *leaf.parent.borrow_mut() = Rc::downgrade(&root);
    println!("left's parent: {:#?}", leaf.parent.borrow().upgrade());


}


