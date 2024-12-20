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