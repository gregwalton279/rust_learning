use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Add;

fn main() {
    // let map = HashMap::new();
    let integer = Point { x: 5, y: 10 };
    let float = Point1 { x: 1.0, y: 4.0 };
    let char = Point { x: 'c', y: 'd' };
    let char = Point1 { x: 'c', y: 'd' };
    float.sum();
    println!("{}", integer.x);
    println!("float.sum() is {:?}", float.sum());

    println!("integer is {:?}", integer);

    let news_article = NewsArticle {
        headline: "1".to_string(),
        location: "2".to_string(),
        author: "3".to_string(),
        content: "4".to_string(),
    };

    let tweet = Tweet {
        username: "Greg Walton".to_string(),
        content: "The book named 'program learning' is published".to_string(),
        retweet: false,
        reply: false,
    };

    println!("news_article summarize is {:?}", news_article.summarize());
    println!("tweet summarize is {:?}", tweet.summarize());
    notify(&tweet);
    notify5(&tweet);
    println!("returns_summarizable().summarize() is {}", returns_summarizable(true).summarize());
    lifetime();

    let novel = "Call me Ishmael. Some years ago: ";
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence
    };
    println!("Important Excerpt is {:?}", i.part);
}

// Option 和 Result
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


struct Point1<T, U> {
    x: T,
    y: U,
}

// 特化：当T和U同类型时候
impl<T: Add + Copy> Point1<T, T> {
    fn sum(&self) -> T::Output {
        self.x + self.y
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub trait Summary {
    // fn summarize(&self) -> String;

    // 添加默认
    fn summarize(&self) -> String {
        String::from("Done")
    }
}


struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    retweet: bool,
    reply: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("Summary for tweet : {}: {}", self.username, self.content)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "one tweet is {}", self.content)
    }
}

// trait 作为参数
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// item1和item2为不同的Summary类型
fn notify3(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}, {}", item1.summarize(), item2.summarize());
}

// item1和item2为相同的Summary类型
fn notify4<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}: {}", item1.summarize(), item2.summarize());
}

// 通过+号 指定多个trait bound
fn notify5(item: &(impl Summary + Display)) {
    println!("Breaking news! {}, {}", item.summarize(), item);
}

// 通过where语句简化trait bound
fn notify6<T: Summary + Display, U: Display + Clone>(t: &T, u: &U) {
    println!("Breaking news! {}, {}", t.summarize(), u);
}
fn notify7<T, U>(t: &T, u: &U) -> ()
where
    T: Display + Summary,
    U: Clone + Display,
{
    println!("Breaking news! {}, {}", t.summarize(), u)
}

// 返回实现trait类型
fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(
            NewsArticle {
                headline: "breaking news".to_string(),
                location: "usa".to_string(),
                author: "Walton".to_string(),
                content: "this is a breaking news from usa".to_string(),
            }
        )
    } else {
        Box::new(Tweet {
            username: "Greg".to_string(),
            content: "publish a message.".to_string(),
            retweet: false,
            reply: false,
        })
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

/// 生命周期Lifetime是一种泛型
/// 生命周期确保引用一直有效

/// 生命周期避免了悬垂引用（野指针）
///

fn lifetime() {
    let r;
    {
        let x = 5;
        r = &x;
    }

    // 超出x的生命周期调用r引用x
    // println!("r: {}", r);


    // 函数中的泛型生命周期
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The longest string is {}", result);
}

// 指定生命周期注解`a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}