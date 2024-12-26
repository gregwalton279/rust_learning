use ch17_object_programming::{Button, Draw, Post, Screen};

fn main() {
    let v = Vec::from([1, 2, 3]);
    let screen = Screen {
        components: vec![
            Box::new(
                SelectBox {
                    width: 75,
                    height: 10,
                    option: vec![
                        String::from("Yes"),
                        String::from("No"),
                        String::from("Maybe"),
                    ]
                }
            ),
            Box::new(
                Button {
                    width: 50,
                    height: 10,
                    label: "Ok".to_string(),
                }
            )
        ]
    };

    screen.run();
    let v = Vec::from([1, 2, 3]);
    
    let v1 = v.clone();
    let s = "Hello";
    let s1 = s.clone();
    let s2 = String::from(s);
    let s3 = s2.clone();
    
    if let [first, rest @ ..] = &v[..]{
        println!("first: {}", first);
        println!("rest: {:?}", rest);
    }

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today".to_string());
    println!("before request review {:?}", post.content());
    assert_eq!("", post.content());

    post.request_review();
    println!("before approve {:?}", post.content());
    assert_eq!("", post.content());

    post.approve();
    println!("approved {:?}", post.content());
    assert_eq!("I ate a salad for lunch today", post.content());
    
}

struct SelectBox {
    width: u32,
    height: u32,
    option: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw selecbox");
    }
}