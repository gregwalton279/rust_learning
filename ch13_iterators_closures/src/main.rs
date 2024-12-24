use std::thread;

fn main() {
    /// 闭包（Closures），一个可以储存在变量里的类似函数的结构
    ///
    /// 迭代器（Iterators），一种处理元素序列的方式
    ///
    /// 闭包（closures）是可以保存进变量或作为参数传递给其他函数的匿名函数。

    let mut store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Green]
    };
    let user_pref1 = Some(ShirtColor::Red);
    let give_away1 = store.give_away(user_pref1);
    println!("The use1 preference {:?} gets {:?}", user_pref1, give_away1);
    let user_pref2 = None;
    let give_away2 = store.give_away(user_pref2);
    println!("The use2 preference {:?} gets {:?}", user_pref2, give_away2);

    let example_closure = |x| x;
    fn example_closure2<T>(x: T) -> T {
        x
    }
    let s = example_closure(String::from("hello 0"));
    let s1 = example_closure2(&s);
    let n = example_closure2(5);
    println!("The example closure {:?} is {:?}", s, s1);
    println!("The example closure2 {:?} is {:?}", s1, n);

    let mut list = vec![1, 2, 3];

    println!("Before define closure {:?}", list);
    let only_borrows = || println!("From closure {:?}", list);

    println!("Before calling closure {:?}", list);
    only_borrows();

    let mut borrow_mutably = || list.push(7);
    // println!("After calling closure {:?}", list);
    borrow_mutably();
    println!("After calling closure {:?}", list);

    /// 闭包可以通过三种方式捕获其环境，
    /// 他们直接对应函数的三种获取参数的方式：
    ///
    /// 获取所有权，可变借用和不可变借用。
    ///
    /// 这三种捕获值的方式被编码为如下三个 Fn trait：
    ///
    /// 1. FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，
    /// environment。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。
    /// 其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
    /// 2. FnMut 获取可变的借用值所以可以改变其环境   // list.iter().fold()
    /// 3. Fn 从其环境获取不可变的借用值
    ///
    thread::spawn(move || println!("From thread {:?}", list))
        .join()
        .unwrap();

    let mut list = [
        Rectangle { width: 10, height: 3 },
        Rectangle { width: 12, height: 5 },
        Rectangle { width: 5, height: 6 },
    ];
    let mut sort_operation = vec![];
    let value = String::from("By key called");
    list.sort_by_key(|r| {
        sort_operation.push(&value);
        r.width
    });

    println!("list is {:#?}", list);

    /// 迭代器

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // for val in v1 { }

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let sum: i32 = v1.iter().sum();

    // y = f(x)
    // y1 = g(x)
    // y2 = g(f(x))
    // g . f
    // y = x
    let hello_world = ["Hello", "from", "world"];
    let hello_flatten: String = hello_world.iter()
        .map(|s| s.chars())
        .flatten()
        .collect();

    let hello_flat_map: String = hello_world.iter()
        .flat_map(|s| s.chars())
        .collect();
    
    let sum = v1.iter()
        .map(|x| x * 2)
        .flat_map(|x| [x, x, x])
        .fold(100, |acc, e| {
            acc + e
        });
    println!("hello flatton is {}",hello_flatten);
    println!("hello flat map is {}",hello_flat_map);
    println!("sum is {}", sum);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
    Green,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn give_away(&mut self, user_preference: Option<ShirtColor>) -> ShirtColor {
        /// 闭包对比
        /// ```rust
        /// fn  add_one   (x: u32) -> u32 { x + 1 }
        /// let add_one = |x: u32| -> u32 { x + 1 };
        /// let add_one = |x|             { x + 1 };
        /// let add_one = |x|               x + 1  ;
        /// ```
        /// 闭包举例:
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        let mut num_green = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Green => num_green += 1,
            }
        }

        if num_red > num_blue {
            if num_red > num_green {
                ShirtColor::Red
            } else {
                ShirtColor::Green
            }
        } else {
            if num_blue > num_green {
                ShirtColor::Blue
            } else {
                ShirtColor::Green
            }
        }
    }
}