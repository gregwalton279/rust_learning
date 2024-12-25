use std::cell::RefCell;
use std::fmt::{write, Debug, Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;
use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let better_list = List::from_array(&[1, 2, 3]);

    println!("list is {}", list);
    println!("better_list is {}", better_list);

    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let a = MyBox::new(x);
    let s = MyBox::new(String::from("Hello"));

    need_str(&s);
    need_str(&(*s)[..]);
    // Vec::new()
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *a);
    assert_eq!(5, *(a.deref()));

    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    drop(c);
    println!("CustomSmartPointers created.");

    let a = Rc::new(RcList::Cons(5, Rc::new(RcList::Cons(10, Rc::new(RcList::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = RcList::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = RcList::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let mut x = Rc::new(vec![1, 2, 3, 4]);
    // x.push(5);  //error
    let x = RefCell::new(vec![1, 2, 3, 4]);
    x.borrow_mut().push(5);
    println!("x is {:?}", x);

    // [5]
    let a = Rc::new(RefRcList::Cons(5, RefCell::new(Rc::new(RefRcList::Nil))));
    println!("a init rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    // [10,a]
    let b = Rc::new((RefRcList::Cons(10, RefCell::new(Rc::clone(&a)))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b init rc count = {}", Rc::strong_count(&a));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // overflowed
    // println!("a next item = {:?}", a.tail());
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn need_str(s: &str) {}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // &self.0
        // self -> &MyBox<T>
        // *self -> MyBox<T>
        // **self -> T
        // &**self -> &T
        // &**self //error
        &self.0
    }
}

#[derive(Debug)]
enum RefRcList {
    Cons(i32, RefCell<Rc<RefRcList>>),
    Nil,
}

impl RefRcList {
    fn tail(&self) -> Option<&RefCell<Rc<RefRcList>>> {
        match self {
            RefRcList::Cons(_, tail) => Some(tail),
            RefRcList::Nil => None,
        }
    }
}

enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}
// 1 -> 2 -> 3 -> Nil
enum List {
    Cons(i32, Box<List>),
    Nil,
}


impl List {
    fn is_empty(&self) -> bool {
        match self {
            Cons(..) => false,
            Nil => true,
        }
    }
    fn from_array(array: &[i32]) -> List {
        if let [head, tail @ ..] = array {
            Cons(*head, Box::new(Self::from_array(tail)))
        } else {
            Nil
        }
    }
}
impl RefRcList {
    fn is_empty(&self) -> bool {
        match self {
            RefRcList::Cons(..) => false,
            RefRcList::Nil => true,
        }
    }
}

impl Display for RefRcList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RefRcList::Cons(head, tail) => write!(f, "{head}{}", if tail.borrow().is_empty() { "" } else { ", " }).and(Display::fmt(&tail.borrow(), f)),
            RefRcList::Nil => Ok(())
        }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cons(head, tail) => write!(f, "{head}{}", if tail.is_empty() { "" } else { ", " }).and(tail.fmt(f)),
            Nil => Ok(())
        }
    }
}