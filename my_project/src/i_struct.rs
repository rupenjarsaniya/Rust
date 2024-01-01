pub fn main() {
    struct User {
        name: String,
        age: u8,
        height: u8,
    }

    let user1 = User {
        name: String::from("Rupen Jarsaniya"),
        age: 22,
        height: 189,
    };

    println!("Name = {}", user1.name);
    println!("Age = {}", user1.age);
    println!("Height = {}", user1.height);

    // Destructuring Fields of a Rust Struct

    let User { name, age, height } = user1;
    println!("{} {} {}", name, age, height);

    // Pass Structs to a Function
    struct Course {
        code: i32,
        name: String,
        level: String,
    }

    fn display_mycourse_info(c: Course) {
        println!("Name:{}, Level:{} ,code: {}", c.name, c.level, c.code);
    }

    let c1 = Course {
        name: String::from("Rust"),
        level: String::from("pro"),
        code: 130,
    };
    display_mycourse_info(c1);

    // Return struct from function
    let c2 = Course {
        name: String::from("Rust"),
        level: String::from("pro"),
        code: 130,
    };
    fn return_struct(c1: Course) -> Course {
        return c1;
    }

    let c: Course = return_struct(c2);
    println!("{} {} {}", c.name, c.code, c.level)
}
