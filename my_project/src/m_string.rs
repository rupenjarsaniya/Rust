pub fn main() {
    // Create empty string
    let a = String::new();
    let b = a.to_string();
    println!("Empty string literal: {}", b);

    let course2 = "Rust Programming";
    println!("string literal : {}.", course2);
    println!("length of my string literal {}.", course2.len());

    // define a String object using from method
    let course3 = String::from("Rust Language");
    println!("string object : {}.", course3);
    println!("length of my string object {}.", course3.len());
    println!("Capacity: {}.", course3.capacity());

    // Finding a Substring
    // define a growable string variable
    let str = String::from("Rust Programming");
    let sub_str = String::from("Rust");
    println!("beginner course in {}.", str);
    println!(
        "{} is a substring of {}: {}.",
        sub_str,
        str,
        str.contains("Rust")
    );

    // Replace a Substring
    let str = String::from("Rust Programming");
    let result = str.replace("Programming", "Language");
    println!("{}", result);

    // Trim a String
    let str = String::from("            Rust         Programming             ");
    println!("Before trim: {} and length is {}", str, str.len());
    let result = str.trim();
    println!("After trim: {} and length is {}", result, result.len());

    // Split
    let str = String::from("Rust Programming");
    let result = str.split("");
    println!("{:?}", result);

    for found in str.split_whitespace() {
        println!("found: {}", found);
    }

    for index in str.split("") {
        println!("index: {}", index);
    }

    for found in str.chars() {
        println!("found: {}", found);
    }

    // Update string
    let mut course = String::from("Rus");
    course.push('t');
    course.push_str(" Programming");
    println!("This is a beginner course in {}.", course);
    println!("Concate {}.", course + " " + &str);

    // Format Macro
    let course = "Rust";
    let _course_type = "beginner course";
    println!("{}", format!("{} {}", course, _course_type));
    println!("{}", format!("{1} {0}", course, _course_type));

    // Slicing
    let slice = &_course_type[5..12];
    println!("Slice : {}", slice);

    // Passing Primitive String - String Literal (&str)
    let course: &str = "Rust Programming";
    display_course_name(&course);

    // Passing Growable String - String Object (String)
    let mut course: String = String::from("Rust Programming");
    course.push_str(" vkfdmk");
    display_course_name(&course);
}

fn display_course_name(my_course: &str) {
    println!("Course : {}", my_course);
}
