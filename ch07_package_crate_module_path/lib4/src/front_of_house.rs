pub mod hosting {
    pub enum Appetizer {
        soup,
        salad {
            a: u8,
            b: u8,
        },
    }
    pub fn add_to_waitlist() {
        /// 绝对路径
        crate::front_of_house::serving::take_order();
    }

    pub fn seat_at_table() {}
}

use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
// 等价于
// use std::io::{self, Write};


mod serving {
    use crate::front_of_house;
    use crate::front_of_house::hosting::Appetizer::soup;

    pub fn take_order() {
        /// 相对路径 self、super、模块名
        /// 需要用use引用front_of_house
        front_of_house::hosting::add_to_waitlist();
        /// 不需要use
        super::hosting::add_to_waitlist();

        let a = crate::front_of_house::hosting::Appetizer::soup;
    }

    fn serve_order() {}

    fn take_payment() {}
}
