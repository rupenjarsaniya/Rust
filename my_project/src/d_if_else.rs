pub fn main() {
    println!("------------------\nIF ELSE\n------------------");

    let age: u8 = 13;

    if age > 18 {
        println!("You can drive");
    } else {
        println!("You can not drive");
    }

    let day: &str = "Monday";

    if day == "Monday" && true {
        println!("Today is Monday");
    } else if day == "Tuesday" {
        println!("Tomorrow is Tuesday");
    } else {
        println!("Lazy Day");
    }
}
