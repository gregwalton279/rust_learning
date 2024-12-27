mod lib;

use std::{fmt, thread};
use std::fmt::{write, Display, Formatter, Pointer};
use std::ops::Add;
use std::process::exit;
use std::ptr::{read_volatile, write_volatile};
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use crate::lib::{Human, Pilot, Wizard};

#[macro_export]
macro_rules! vec2 {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    println!("num is {}", num);
    println!("r1 is {:?}", r1);
    unsafe {
        println!("*r1 is {}", *r1);
        println!("*r2 is {}", *r2);
    }
    unsafe { *r2 = 3; }
    println!("num is {}", num);

    println!("r1 is {:?}", r1);
    unsafe {
        println!("*r1 is {}", *r1);
        println!("*r2 is {}", *r2);
    }

    // unsafe 作为一个表达式
    let num2 = unsafe { *r2 };
    println!("num2 is {}", num2);

    let address = 0x012345usize;
    let r = address as *const i32;
    // unsafe { read_volatile(r); }
    // unsafe { write_volatile(address,  8usize); }
    println!("r is {:?}", r);

    unsafe fn dangerous() {}

    unsafe { dangerous(); }

    /// 将不安全代码封装进安全函数
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3); // 3: index
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    println!("{:?}", unsafe { a });

    /// 使用extern函数调用外部代码
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("name is : {}", HELLO_WORLD);

    add_to_count(3);
    /// 访问一个可变静态变量，是不安全的
    /// 原因是考虑到，拥有多个线程访问counter时，可能导致数据竞争
    unsafe { println!("Counter is : {}", COUNTER); }

    /// ### 默认泛型类型参数和运算符重载
    /// 运算符重载
    ///
    /// 实现了Add 的 point 后可以相加了
    /// 
    
    assert_eq!(Point { x: 1, y: 3 } + Point { x: 2, y: 2 }, Point { x: 3, y: 5 });

    let human = lib::Human;
    Pilot::fly(&human);
    Wizard::fly(&human);
    human.fly();

    /// Pilot::fly 是一个function
    need_a_funtion(Pilot::fly);

    /// 完全限定语法
    /// ```rust
    /// <Type as Trait>::function(receiver_if_method, next_arg, ...);
    /// ```
    println!("Animal name is {}", <Dog as Animal>::baby_name());

    let w = Wrapper(vec![String::from("hello"), String::from("hello"), String::from("world"), String::from("world")]);
    println!("w = {}", w);

    let my_closure = Box::new(|| {
        println!("Hello from the closure!");
    });

    crate::lib::takes_long_type(my_closure);

    // 等待一会儿以确保线程完成
    thread::sleep(std::time::Duration::from_secs(1));


    let s: String = String::from("Hello, world!"); // 创建一个 String
    let slice: &str = &s[0..5]; // 创建一个 &str 切片

    println!("String: {}", s); // 输出: String: Hello, world!
    println!("Slice: {}", slice); // 输出: Slice: Hello

    // 修改 String
    let mut s_mut = s;
    s_mut.push_str(" Rust!"); // 追加字符串
    println!("Modified String: {}", s_mut); // 输出: Modified String: Hello, world! Rust!

    println!("returns a funtions = {}", returns_a_function()(2));
    println!("returns a funtions = {}", returns_a_function2()(5));
    let v = vec2![6, 2, 3];
    let v1 = vec![1, 2, 3, ];
    let v2 = vec![4; 5];
    
    Pancakes::hello_macro();
    
    
}

#[derive(HelloMacro)]
struct Pancakes;


fn add_one(x: i32) -> i32 {
    x + 1
}

fn returns_a_function() -> fn(i32) -> i32 {
    add_one
}

fn returns_a_function2() -> fn(i32) -> i32 {
    |x| x + 1
}
fn need_a_funtion<F>(f: F)
where
    F: FnOnce(&Human) -> (),
{
    let person = Human;
    f(&person);
}

extern "C" {
    fn abs(input: i32) -> i32;
}

/// 从其它语言调用Rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

/// 全局变量，也叫静态变量
static HELLO_WORLD: &str = "Hello, World!";

static mut COUNTER: u32 = 0;
/// 访问一个可变静态变量，是不安全的
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

/// ### 实现不安全 trait
/// unsafe 的另一个操作用例是实现不安全 trait。
/// 当 trait 中至少有一个方法中包含编译器无法验证的不变式（invariant）时 trait 是不安全的。
/// 可以在 trait 之前增加 unsafe 关键字将 trait 声明为 unsafe，同时 trait 的实现也必须标记为 unsafe

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Dog;

trait Animal {
    fn baby_name() -> String;
}
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

/// 接口继承
/// 父 trait 用于在另一个 trait 中使用某 trait 的功能
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        // self.fmt()
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
/// 需要先实现fmt，再实现outlinePrint
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl OutlinePrint for Point {
    fn outline_print(&self) {
        self.outline_print();
    }
}

/// newtype 模式用于在外部类型上实现外部 trait
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

/// never type  从不返回的类型
/// pub fn exit(code: i32) -> ! {
fn ext() {
    exit(1);
}

fn generate<T>(t: T) {}
/// 实际上是如下
fn generate_real<T: Sized>(t: T) {}
/// 特殊语法可以放宽限制
fn generate_special<T: ?Sized>(t: &T) {}
