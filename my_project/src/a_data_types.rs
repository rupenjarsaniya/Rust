pub fn main() {
    println!("------------------\nDATA TYPES\n------------------");
    // U can not store negative value, I can store negative and positive values
    let n: i32 = 29;
    let n1: i32 = -29;
    let n2: u32 = 20;
    // let n3: u32 = -20;

    let n4: f32 = 10.01; //float

    let b1: bool = false; //bool

    let c1: char = 'A'; // character

    let c2: char = '&'; // string
    let c3: &str = "&"; // string

    println!(
        "n = {}, n1 = {}, n2 = {}, n4 = {}, b1 = {}, c1 = {}, c2 = {}, c3 = {}",
        n, n1, n2, n4, b1, c1, c2, c3
    );
}
