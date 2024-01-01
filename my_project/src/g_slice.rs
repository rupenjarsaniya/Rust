pub fn main() {
    println!("------------------\nSLICE\n------------------");

    let numbers = [1, 2, 3, 4, 5];
    let slice1 = &numbers[..3]; // First 3
    let slice2 = &numbers[2..]; // 2 is index which is ommiting from array

    println!("slice1 = {:?}", slice1);
    println!("slice2 = {:?}", slice2);

    let mut colors = ["Red", "Green", "Blue"];
    let color1 = &mut colors[1..];

    println!("color1 = {:?}", color1);

    color1[1] = "White";

    println!("color1 = {:?}", color1);
}
