fn main() {
    /// ## 所有可能会用到模式的位置
    /// ### 1. match 分支
    /// ```rust
    /// match VALUE {
    ///     PATTERN => EXPRESSION,
    ///     PATTERN => EXPRESSION,
    ///     PATTERN => EXPRESSION,
    /// }
    /// ```
    /// match 表达式必须是 穷尽（exhaustive）
    ///
    ///  _ 可以匹配所有情况，不过它从不绑定任何变量。
    ///
    /// ### 2. if let 条件表达式
    /// if let 可以对应一个可选的带有代码的 else 在 if let 中的模式不匹配时运行。
  
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
    /// ### 3. while let 条件循环
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    
    /// ### 4. for 循环
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    
    /// ### 5. let语句
    /// `let PATTERN = EXPRESSION;`
    let x = 5;
    let (x, y, z) = (1, 2, 3);
    
    /// ### 6. 函数参数
    fn foo(x: i32) {
        // 代码
    }

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);
    
    /// ### 模式有两种形式：refutable（可反驳的）和 irrefutable（不可反驳的）。
    /// 能匹配任何传递的可能值的模式被称为是 不可反驳的（irrefutable）。如：let x = 5;
    /// 语句中的 x，因为 x 可以匹配任何值所以不可能会失败。
    /// 
    /// 对某些可能的值进行匹配会失败的模式被称为是 可反驳的（refutable）。
    /// 如：if let Some(x) = a_value 表达式中的 Some(x).
    /// 如果变量 a_value 中的值是 None 而不是 Some，那么 Some(x) 模式不能匹配。
    /// 
    /// 函数参数、 let 语句和 for 循环只能接受不可反驳的模式，
    /// 因为通过不匹配的值程序无法进行有意义的工作。
    /// 
    /// if let 和 while let 表达式被限制为只能接受可反驳的模式，
    /// 因为根据定义他们意在处理可能的失败：
    /// 条件表达式的功能就是根据成功或失败执行不同的操作。
    /// 
    /// let Some(x) = some_option_value; // error
    struct GetOption {
        option: Option<i32>,
    }
    impl GetOption {
        fn new(option: Option<i32>) -> GetOption {
            GetOption { option }
        }
        fn option(&self) -> Option<i32> {
            self.option
        }
    }
    // let k = GetOption::new(Some(1)).option();
    let k = GetOption::new(None).option();
    println!("get k = {:?}", k);
    if let Some(x) = &k {
        println!("get Option = {}", x);
    }else {
        println!("get Option else None = {}", x);
    }
    println!("k = {:?}", k);
    
    /// ## 模式语法
    /// ### 1. 匹配字面量
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    
    /// ### 2. 匹配命名变量
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // x会匹配到此
        _ => println!("Default case, x = {:?}", x), // 非Some()值匹配到此
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
    
    /// ### 3. 多个模式
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    
    /// ### 4. 通过 ..= 匹配值的范围
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }


    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
    
    /// ### 5. 解构并分解值
    /// ### 解构struct
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
    
    /// ### 解构枚举

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }
    
    /// ### 解构嵌套的结构体和枚举
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h,
                s,
                v
            )
        }
        _ => ()
    }
    
    /// ### 解构结构体和元组
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
    
    /// ## 忽略模式中的值
    /// 1. 使用 _ 忽略整个值
    fn foo2(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    foo2(3, 4);
    /// 2. 使用嵌套的 _ 忽略部分值
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }
    
    /// ### 通过在名字前以一个下划线开头来忽略未使用的变量
    let _x = 5;
    let y = 10;
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    // println!("{:?}", s);  // error s已经被_s move了
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);  // 使用_ 不会move
    
    /// ### 用 .. 忽略剩余值
    struct Point2 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point2 { x: 0, y: 0, z: 0 };

    match origin {
        Point2 { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }
    
    /// ### 匹配守卫提供的额外条件
    let num = Some(3);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x), // 这里
        Some(x) if x < 4 => println!("less than four: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"), // (4 | 5 | 6) if y => ...
        _ => println!("no"),
    }
    
    /// ### @ 绑定
    /// at 运算符（@）允许我们在创建一个存放值的变量的同时测试其值是否匹配模式。
    /// 使用 @ 可以在一个模式中同时测试和保存变量值。
    enum Message3 {
        Hello { id: i32 },
    }

    let msg = Message3::Hello { id: 5 };

    match msg {
        Message3::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message3::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
    
    
}
