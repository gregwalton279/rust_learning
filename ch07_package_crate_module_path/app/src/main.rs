use lib2::Hello;
use lib1::Hello as Hello1;
use lib3::front_of_house::hosting;

fn main() {
    Hello();
    Hello1();
    println!("Hello, world!");
    hosting::add_to_waitlist();
}