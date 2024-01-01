pub fn main() {
    println!("------------------\nTYPE CASTING\n------------------");

    // Decimal to Integer
    let decimal: f32 = 10.90;
    let integer = decimal as i16;
    println!("Decimal = {}, Integer = {}", decimal, integer);

    // Character to Integer
    let a: char = 'r';
    let b: i16 = a as i16;
    println!("A = {}, B = {}", a, b);

    // Character to Integer - only `u8` can be cast as `char`
    let c: u8 = 114;
    let d: char = c as char;
    println!("C = {}, D = {}", c, d);

    // Boolean to Integer
    let e: bool = true;
    let f: bool = false;
    let e1 = e as i16;
    let f1 = f as i16;

    println!("e = {}, f = {}, e1 = {}, f1 = {}", e, f, e1, f1);
}
