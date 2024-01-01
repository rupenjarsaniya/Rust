pub fn main() {
    // Create Vectors
    let name: Vec<&str> = vec!["Rupen", "Rust", "Contract"];
    println!("{:?}", name);

    // Access an Element of a Vector
    println!("{}", name[0]);
    match name.get(2) {
        Some(x) => println!("Value at given index: {}", x),
        None => println!("Sorry, you are accessing a value out of bound"),
    }

    // Print the Vector
    println!("Print using for loop");
    let mut index = 0;
    for i in name {
        println!("{} -> {}", index, i);
        index = index + 1;
    }

    // Methods of Vectors
    let mut my_vec = Vec::new();
    println!("Empty Vector = {:?}", my_vec);

    my_vec.push(1);
    my_vec.push(3);
    my_vec.push(2);
    println!("Vector = {:?}", my_vec);

    my_vec.pop();
    println!("Vector = {:?}", my_vec);
    my_vec.remove(0);
    println!("Vector = {:?}", my_vec);

    println!("Does my vector contains 1 : {}", my_vec.contains(&3));

    // Print length and capacity
    let strs: Vec<&str> = vec!["abc", "def"];
    println!("Length of the vector : {}", strs.len());
    println!("Capacity of vector: {}", strs.capacity());

    // Iterate Using .iter() Built-in Method
    let mut a = vec![1, 2, 3, 4, 5];
    let value = 2;
    let index = a.iter().position(|&r| r == value).unwrap();
    a.remove(index);
    println!("Updated Vector: {:?}", a);

    // Loop Through the Values
    let mut index = 0;
    for i in a.iter() {
        // it works even if .iter() is not written
        println!("Element at index {}:{} ", index, i);
        index = index + 1;
    }

    // Get Slice
    let slice: &[i32] = &a[2..4];
    println!("Slice of the vector : {:?}", slice);

    let colors = vec!["blue", "red", "green"];
    println!("first color = {:?}", colors.get(0));
    println!("first color = {:?}", colors[0]);
}
