pub fn main() {
    println!("------------------\nOPERATORS\n------------------");

    let mut a = 17;
    let b = 4;

    println!(
        "sum = {}, Minus = {}, Multiply = {}, Division = {}, Reminder = {}",
        a + b,
        a - b,
        a * b,
        a / b,
        a % b
    );

    println!(
        "Greater = {}, Greater than queal = {}, Less than = {}, Less than queal = {}",
        a > b,
        a >= b,
        a < b,
        a <= b
    );

    a += 2;
    println!("a = {}", a);
    a -= 2;
    println!("a = {}", a);

    println!("Equal = {}, Not equal = {}", a == b, a != b);

    println!("&& = {}", true && false);
    println!("&& = {}", true && true);
    println!("&& = {}", false && false);
    println!("|| = {}", true || true);
    println!("|| = {}", true || false);
    println!("|| = {}", false || false);
    println!("! = {}", !true);
}
