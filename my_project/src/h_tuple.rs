pub fn main() {
    // Mutable Tuple & Rust Tuple with Data Type
    let mut tuple: (&str, i8, i8, f32) = ("hello", 1, 2, 3.14);

    println!("Original Tuple = {:?}", tuple);
    // Accessing Elements in a Tuple
    println!("Index1 = {}", tuple.0);
    println!("Index1 = {}", tuple.1);

    tuple.1 = 'a' as i8;
    println!("Index1 = {}", tuple.1);
    println!("After Change Tuple Looks Like = {:?}", tuple);

    // Destructuring a Tuple
    let info = ("Rupen", 21, 1);
    let (name, age, experience) = info;

    println!("Name = {}", name);
    println!("Age = {}", age);
    println!("Experience = {}", experience);

    let (x, y) = coordinate();
    println!("x = {:?}, y = {:?}", x, y);
}

fn coordinate() -> (i32, i32) {
    (1, 7)
}
