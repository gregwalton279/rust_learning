pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// ### cargo test cmd
/// ```bash
///
/// cargo test --help
///
/// // 将另外一部分传递给生成的测试二进制文件
/// cargo test -- --help
///
/// // 将测试线程设置为 1
/// cargo test -- --test-threads=1
///
/// // 告知 Rust 显示通过测试的输出
/// cargo test -- --show-output
///
/// // 通过指定名字来运行部分测试
/// cargo test can_hold_test
///
/// // 运行被忽略的测试
/// cargo test -- --ignored
/// 
/// // 运行某个测试文件中的所有测试
/// cargo test --test integration_test
/// ```


/// ### 单元测试
/// 单元测试与他们要测试的代码共同存放在位于 src 目录下相同的文件中。
/// 规范是在每个文件中创建包含测试函数的 tests 模块，并使用 cfg(test) 标注模块。
/// 可以直接测试私有函数
///
/// ### 集成测试
/// 集成测试对于你需要测试的库来说完全是外部的。
/// 同其他使用库的代码一样使用库文件，也就是说它们只能调用一部分库中的公有 API 。
/// 需要在项目根目录创建一个 tests 目录，与 src 同级。
/// tests 文件夹在 Cargo 中是一个特殊的文件夹， Cargo 只会在运行 cargo test 时编译这个目录中的文件。
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn expensive_test() {
        // 需要运行一个小时的代码
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    /// 这样编写测试来返回 Result<T, E> 就可以在函数体中使用问号运算符，
    /// 如此可以方便地编写测试，如果其中的任何操作返回 Err 值，则测试将失败。
    ///
    /// 你不能在使用 Result<T, E> 的测试中使用 #[should_panic] 注解。要断言操作返回 Err 值，
    /// 不要在 Result<T, E> 值上使用问号运算符。相反，请使用 assert!(value.is_err())。
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            return Ok(());
        } else {
            return Err(String::from("two plus two maths"));
        }
    }

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_ne!(result, 6);
        assert!(result == 4, "result was wrong result: {}", result);
    }

    #[test]
    fn can_hold_test() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "Greeting result is {result}", );
    }

    #[test]
    #[should_panic]
    fn smaller_than_1() {
        Guess::new(0);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        } else {
            Guess { value }
        }
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}
