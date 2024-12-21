pub fn run(){
    let mut v: Vec<i32> = Vec::new();
    // 通常使用初始值创建,Rust会推断类型
    let mut v2 = vec![1, 2, 3];
    let _v2 = vec!["hello", "world"];

    v.push(5);
    v.append(&mut v2);
    v.pop();

    println!("{:?}", v.iter());

    for e in v.iter() {
        println!("{}", e);
    }

    println!("v len: {}, v2 len: {}", v.len(), v2.len());

    // 使用get获取
    let i = v.get(1);
    // 会报错，作用域不能在使用v的变量前更改v的值
    // v.push(100);
    println!("{:?}", i);

    // 与get方法的返回值类型不同
    let third: &i32 = &v[2];
    println!("The third element is {}, v len: {}", third, v.len());

    let j = v.get(100);
    let jj = j.map_or(-1, |&x| x * 2);
    println!("{}", jj);
    match j {
        None => println!("j is None"),
        Some(value) => println!("value is {}", value),
    }

    // 遍历vector
    for i in &v {
        println!("{}", i);
    }
    println!("1 {:?}", v);
    // 遍历修改vector,*是解引用
    for i in &mut v {
        *i += 50;
    }
    println!("2 {:?}", v);


    // 使用枚举enum，来使vector能够存储多种类型
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    for e in &row {
        match e {
            SpreadsheetCell::Int(i) => println!("{}", i),
            SpreadsheetCell::Float(f) => println!("{}", f),
            SpreadsheetCell::Text(s) => println!("{}", s),
        }
    }
}