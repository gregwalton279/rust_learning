use std::ops::Add;
use std::thread;

/// 为什么Iterator的trait使用【关联类型】Iterm，而不是用泛型？？
/// ```rust
/// pub trait Iterator<T>{
///     fn next(&mut self)-> Option<T>;
/// }
/// ```
/// 因为如果<T>不一样的话，对于Iterator，都是不同的类型，
/// 所以Counter可以实现无数种类型的Iterator
/// 而采用【关联类型】type Item;则只能实现一个Iterator。

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

struct Counter {
    count: u32,
}

/// new type模式

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

/// ### 完全限定语法与消歧义：调用相同名称的方法
/// Rust 既不能避免一个 trait 与另一个 trait 拥有相同名称的方法，
///
/// 也不能阻止为同一类型同时实现这两个 trait。
///
/// 甚至直接在类型上实现开始已经有的同名方法也是可能的！
pub trait Pilot {
    fn fly(&self);
}

pub trait Wizard {
    fn fly(&self);
}

pub(crate) struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    pub fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

/// Rust 中有三种主要的函数 trait：Fn、FnMut 和 FnOnce。它们的主要区别在于对闭包捕获环境的方式。
/// 1. Fn Trait
/// * 定义：Fn trait 代表可以被调用的函数或闭包，并且可以多次调用而不改变其状态。
/// * 用途：通常用于需要传递函数作为参数的场景，比如在高阶函数中。
/// 2. FnMut Trait
/// * 定义：FnMut trait 表示可以被调用且可能会修改其捕获的环境状态的闭包。
/// * 用途：用于需要修改状态的场景。
/// 3. FnOnce Trait
/// * 定义：FnOnce trait 表示可以被调用一次的闭包，通常用于消费其捕获的环境。
/// * 用途：适用于需要移动捕获变量的场景。
/// ---
/// * Fn：可多次调用，不修改状态.
/// * FnMut：可多次调用，可能修改状态。
/// * FnOnce：只能调用一次，可能消耗捕获的变量。
type Thunk = Box<dyn Pilot + Send + 'static>;
fn better_takes_long_type(f: Thunk){}
pub(crate) fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    // 在新的线程中调用闭包
    thread::spawn(move || {
        f(); // 调用闭包
    });
}

fn main() {}
