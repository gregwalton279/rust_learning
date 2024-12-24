use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    result.iter().for_each(|x| println!("search result {}", x));
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub cmd: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            Err(format!("Not enough arguments"))
        } else {
            let query = args[1].clone();
            let file_path = args[2].clone();
            let cmd = args[0].clone();
            let ignore_case = env::var("IGNORE_CASE").map_or(false, |value| {
                match value.as_str() {
                    "true" => true,
                    "false" => false,
                    "0" => false,
                    "1" => true,
                    _ => false
                }
            });

            println!("ignore case is {}", ignore_case);
            Ok(
                Config {
                    query,
                    file_path,
                    cmd,
                    ignore_case,
                }
            )
        }
    }
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().fold(Vec::new(), |mut acc, line| {
        if line.contains(query) {
            acc.push(line);
        }
        acc
    })
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

/// 测试驱动开发（Test Driven Development, TDD）的模式来逐步增加搜索逻辑。
/// 这是一个软件开发技术，它遵循如下步骤：
///
/// 1. 编写一个失败的测试，并运行它以确保它失败的原因是你所期望的。
/// 2. 编写或修改足够的代码来使新的测试通过。
/// 3. 重构刚刚增加或修改的代码，并确保测试仍然能通过。
/// 4. 从步骤 1 开始重复！
///

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let new_contents = indoc! {
            "\
            Rust:
            safe, fast, productive.
            Pick three."
        };

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, new_contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = indoc! {
            "\
            Rust:
            safe, fast, productive.
            Pick three.
            Trust me."
        };

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
