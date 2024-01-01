pub fn main() {
    let numbers = [2, 1, 17, 99, 34, 56];
    println!("Numbers: {:?}", numbers);

    for i in numbers.iter() {
        println!("Number: {}", i);
    }

    // next() Method of an Iterator in Rust

    let colors = vec!["Red", "Yellow", "Green"];
    let mut colors_iterator = colors.iter();
    println!("colors iterator = {:?}", colors_iterator);

    println!("{:?}", colors_iterator.next());
    println!("{:?}", colors_iterator.next());
    println!("{:?}", colors_iterator.next());
    println!("{:?}", colors_iterator.next());

    println!("----------------------------------------------------------------");
    // 1. Using iter() method
    for color in colors.iter() {
        println!("{}", color);
    }

    println!("Collection is here: {:?}", colors);

    println!("----------------------------------------------------------------");
    // 2. Using into_iter() method
    for color in colors.into_iter() {
        // the items in the collection move into this scope
        println!("{}", color);
    }
    // println!("Collection is here: {:?}", colors); --> error

    println!("----------------------------------------------------------------");
    // 3. Using iter_mut() method
    let mut colors = vec!["Red", "Yellow", "Green"];
    for color in colors.iter_mut() {
        // modify the item in the collection
        *color = "Black";
        println!("{}", color);
    }
    println!("colors = {:?}", colors);

    // Iterator Adapters in Rust
    let numbers = vec![1, 2, 3];

    // using the map iterator adapter
    let even_numbers: Vec<i32> = numbers.iter().map(|i| i * 2).collect();

    println!("numbers = {:?}", numbers);
    println!("even_numbers = {:?}", even_numbers);
}
