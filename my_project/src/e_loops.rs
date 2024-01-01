pub fn main() {
    println!("------------------\nLOOPS\n------------------");

    let mut i = 1;

    // Loop
    loop {
        println!("Hello, loop! = {}", i);
        i = i + 1;
        if i == 1000 {
            break;
        }
    }

    while i < 1100 {
        println!("Hello, while loop! = {}", i);
        i = i + 1;
    }

    for j in 0..10 {
        if j == 7 {
            break;
        }
        if j == 2 {
            continue;
        }
        println!("Hello, for loop! = {}", j);
    }
}
