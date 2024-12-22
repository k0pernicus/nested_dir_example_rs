use crate::something::inside;

pub fn hello_from_inside() {
    let i: inside::Inside = inside::Inside {};
    i.say_hello();
}
