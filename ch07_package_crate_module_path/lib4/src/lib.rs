pub mod front_of_house;
pub mod new_front_house;

pub use crate::front_of_house::hosting as hosting2;
fn eating_at_restaurant() {
    hosting2::Appetizer::salad { a: 0, b: 0 };
}
