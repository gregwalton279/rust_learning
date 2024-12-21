/// > 字符串困难有三个理由：
/// > 1. Rust 倾向于确保暴露出可能的错误，
/// > 2. 字符串是比很多开发者所想象的要更为复杂的数据结构，
/// > 3. 以及 UTF-8。
///
/// 字符串就是作为字节的集合外加一些方法实现的，当这些字节被解释为文本时，这些方法提供了实用的功能。
///
/// ## 什么是字符串？
/// Rust 的核心语言中只有一种字符串类型：str，字符串 slice。
/// 它通常以被借用的形式出现，&str。
///
/// **字符串 slice** ：它们是一些储存在别处的 UTF-8 编码字符串数据的引用。
/// 比如字符串字面量被储存在程序的二进制输出中，字符串 slice 也是如此。
///
/// 称作 **String** 的类型是由标准库提供的，而没有写进核心语言部分。
/// 它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。
///
/// 谈到 Rust 的 “字符串”时，它们通常指的是 String 和字符串 slice &str 类型，而不仅仅是其中之一。
/// String 和字符串 slice 都是 UTF-8 编码。

pub fn run(){
    /// ## 新建字符串
    /// 很多 Vec 可用的操作在 String 中同样可用，
    /// 从 new 函数创建字符串开始
    /// 字符串是 UTF-8 编码的，所以可以包含任何正确编码的数据
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    let s2 = "initial contents".to_string();

    let s3 = String::from("Hello, ");

    /// ## 更新字符串
    ///
    let mut s4 = s + s2.as_str(); // 注意 s被移动了，不能继续使用
    println!("s4 is {}, s2 is {}", s4, s2);

    /// push 和 push_str
    s4.push('x');
    s4.push('中');
    s4.push_str(", world");
    println!("s4 is {}", s4);

    /// format!
    ///
    let s5 = format!("{s3}: {s2}");
    println!("s5 is {}", s5);

    /// 索引字符串
    /// 不能使用索引 [0] 获取字符。
    /// 有三种相关方式可以理解字符串：字节、标量值和字形簇（最接近人们眼中 字母 的概念）。
    ///
    /// 1. 字符串 slice
    /// 索引字符串通常是一个坏点子，因为字符串索引应该返回的类型是不明确的：
    ///     * 字节值、
    ///     * 字符、
    ///     * 字形簇
    ///     * 或者字符串 slice。
    ///
    /// 但rust允许，可以这么做。
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    /// 遍历字符串的方法
    /// 2. 获取字符
    ///
    /// 输出结果：
    /// ```bash
    /// न
    /// म
    /// स
    /// ्
    /// त
    /// े
    /// ```

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    /// 3. 获取原始字节
    ///
    /// 输出结果：
    /// ```bash
    /// 224
    /// 164
    /// // --snip--
    /// 165
    /// 135
    /// ```
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }


}