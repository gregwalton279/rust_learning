use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, Write};

fn main() {
    // let v = vec![1, 2, 3];
    // v[99];
    let file_name = String::from("hello1.txt");
    let  greeting_file_result = File::options().write(true).read(true).open(&file_name);

    // greeting_file_result.unwrap();
    // greeting_file_result.expect("aaa");
    //  greeting_file_result.expect_err("File does not exist");  //希望是错误时使用，单元测试。unittest
    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {  
            ErrorKind::NotFound => match File::create(&file_name) {
                Ok(file) => file,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            }
            other_error => panic!("Problem opening the file: {:?}", other_error),
        }
    };

    let write_result = greeting_file.write("skskskskskkskksk 22222".as_bytes());
    match write_result {
        Ok(size) => println!("Wrote {} bytes", size),
        Err(error) => println!("Problem writing to file: {:?}", error),
    }
    
    
    // if let Ok(mut file) = greeting_file_result {
    //     let mut str = String::new();
    //     let size = file.read_to_string(&mut str).unwrap_or(0);
    //     println!("{str},{size}");
    // }else { 
    //     println!("File could not be opened");
    // }
    
    println!("username : {:?}", read_username_from_file1());
}

fn panic(){
    println!("Hello, world!");
    println!{"hello world!"};
    panic!{}
}

// 传播错误
fn read_username_from_file1() -> Result<String, std::io::Error> {
    let mut username = String::new();
    //
    // File::open("hello.txt")?.read_to_string(&mut username).map(|_| username)
   
    // 函数式编程
    File::open("hello.txt").and_then(|mut f|f.read_to_string(&mut username).map(|_| username))
}


fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut username_file = File::open("hello.txt")?;
    // let username_file_result = File::open("hello.txt");
    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    
    let mut username = String::new();

    username_file.read_to_string(&mut username).map(|_| username)
    // username_file.read_to_string(&mut username)?;
    // Ok(username)
    
    
    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }
}