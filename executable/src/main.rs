use main_library::{add_two_f32, get_greet_string, Person};
fn main() {
    let person = Person {
        name: "Aaron".into(),
        age: 23,
        children: None,
    };

    println!("{}", get_greet_string(&person));

    let a: f32 = 4.0;
    let b: f32 = 2.0;
    println!("{} + {} = {}", a, b, add_two_f32(a, b));
}
