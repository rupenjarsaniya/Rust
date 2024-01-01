use std::collections::HashSet;

pub fn main() {
    let mut colors: HashSet<String> = HashSet::new();
    println!("Hashset: {:?}", colors);

    // Add Values
    colors.insert(String::from("Red"));
    colors.insert(String::from("Yellow"));
    println!("Hashset: {:?}", colors);

    // Check Values
    if colors.contains("Red") {
        println!("Red is here");
    }

    // Remove Values
    colors.remove("Red");
    println!("Hashset: {:?}", colors);
    if colors.contains("Red") {
        println!("Red is here");
    } else {
        println!("Red is not here");
    }

    // Iterate over Values
    let mut c = colors.clone();
    c.insert(String::from("Red"));
    c.insert(String::from("Yellow"));
    c.insert(String::from("Blue"));
    c.insert(String::from("Aqua"));
    for color in c {
        println!("Colors: {:?}", color);
    }

    // Method of HashSet
    println!("Length = {:?}", colors.len());
    println!("Drain = {:?}", colors.drain());

    println!("Is Empty = {:?}", colors.is_empty());
    colors.clear();
    println!("Is Empty = {:?}", colors.is_empty());

    // Set operations
    // 1. Union of two Sets
    let hashset1 = HashSet::from([2, 7, 8]);
    let hashset2 = HashSet::from([1, 2, 7]);

    let result: HashSet<_> = hashset1.union(&hashset2).collect();
    println!("Result = {:?}", result);

    // 2. Intersection of two Sets
    let result: HashSet<_> = hashset1.intersection(&hashset2).collect();
    println!("Result = {:?}", result);

    // 3. Difference between two Sets
    let result: HashSet<_> = hashset1.difference(&hashset2).collect();
    println!("Result according to hashset1 = {:?}", result);

    // 4. Symmetric Difference between two Sets
    let result: HashSet<_> = hashset1.symmetric_difference(&hashset2).collect();
    println!("Result according to hashset2 = {:?}", result);
}
