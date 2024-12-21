use std::collections::{HashMap};
use std::ops::AddAssign;

pub fn run() {
    /// 新建hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Red"), 5);

    /// 访问hash map
    let team_name = String::from("Blue");
    let score1 = scores[&"Yellow".to_string()];
    let score2 = scores.get(&team_name);

    println!("score1 = {}, score2 = {:?}", score1, score2);

    for (k, v) in &scores {
        println!("k = {}, v = {}", k, v);
    }

    /// 更新hash map
    scores.insert(String::from("Blue"), 25);

    scores.entry(String::from("Yellow")).and_modify(|e| *e = 250);

    let color_of_red = scores.get_mut(&String::from("Red"));
    match color_of_red {
        None => (),
        Some(v) => { *v = 1000 }
    }


    /// 只有key而没有value时，插入default
    scores.entry(String::from("Blue")).or_insert(100);
    println!("scores = {:?}", scores);

    /// 练习一：
    ///
    /// 给定一系列数字，使用 vector 并返回这个列表的平均数（mean, average）、中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助）。
    let numbers = vec![1, 223, 8, 45, 43, 8, 3, 34];
    let mean_average = get_mean_average(&numbers);
    let median = get_median(&numbers);
    let mode = get_mode(&numbers);
    let mode2 = get_mode2(&numbers);
    let mode3 = get_mode3(&numbers);

    println!("mean_average = {:?}", mean_average);
    println!("median = {:?}", median);
    println!("mode = {:?}", mode);
    println!("mode2 = {:?}", mode2);
    println!("mode3 = {:?}", mode3);
}

fn get_mode(numbers: &Vec<i32>) -> Option<i32> {
    if numbers.is_empty() { return None; }
    let mut number_count = HashMap::new();
    for number in numbers {
        let count = number_count.entry(number).or_insert(0);
        *count += 1;
    }
    let mut max = 0;
    let mut count = 0;
    for (key, value) in number_count {
        if value > count {
            max = *key;
            count = value;
        }
    }
    Some(max)
}

fn get_mode2(numbers: &Vec<i32>) -> Option<i32> {
    let a = numbers
        .iter()
        .fold(HashMap::new(), |mut acc, i| {
            acc.entry(i).or_insert(0).add_assign(1);
            acc
        });
        println!("aaa is {:?}",a);

        a.iter()
        .reduce(|current, next| {
            if next.1 > current.1 {
                next
            } else {
                current
            }
        })
        .map(|(k)| **k.0)
}

fn get_mode3(numbers: &Vec<i32>) -> Option<i32> {
    numbers
        .iter()
        .fold(HashMap::new(), |mut acc, i| {
            acc.entry(i).or_insert(1).add_assign(1);
            acc
        })
        .iter()
        .max_by(|a, b| {(*a).1.cmp((*b).1)})
        .map(|(k)| **k.0)
}

fn get_median(numbers: &Vec<i32>) -> Option<i32> {
    if numbers.is_empty() { return None; }
    let mut new_numbers = numbers.to_vec();
    new_numbers.sort();
    println!("{:?}", new_numbers);
    let pos = new_numbers.len() / 2;
    println!("pos = {}", pos);
    Some(new_numbers[pos])
}

fn get_mean_average(numbers: &Vec<i32>) -> Option<f64> {
    if numbers.is_empty() { return None; }
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }

    Some(sum as f64 / numbers.len() as f64)
}