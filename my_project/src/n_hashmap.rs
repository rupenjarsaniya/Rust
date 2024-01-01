use std::collections::HashMap;

pub fn main() {
    let mut fruits: HashMap<i32, String> = HashMap::new();
    println!("{:?}", fruits);

    // Add Elements
    fruits.insert(1, String::from("Kiwi"));
    fruits.insert(2, String::from("Cherry"));
    fruits.insert(3, String::from("New Cherry"));
    println!("{:?}", fruits);

    // Access Values
    let fruit_acc = fruits.get(&1);
    let fruit_acc1 = fruits.get(&10);
    println!("Fruit = {:?}", fruit_acc);
    println!("Fruit = {:?}", fruit_acc1);

    // Remove Elements
    fruits.remove(&1);
    fruits.remove(&2);
    println!("{:?}", fruits);

    // Change Elements
    fruits.insert(1, "Banana".to_string());
    println!("{:?}", fruits);
    fruits.insert(1, "Appple".to_string());
    println!("{:?}", fruits);

    // Methods of hashmap
    println!("Length = {}", fruits.len());
    println!("Contains key = {}", fruits.contains_key(&1));

    let mut index = 0;
    for i in fruits.iter() {
        println!("Fruits = {} {:?}", index, i);
        index = index + 1;
    }

    let mut index = 0;
    for i in fruits.values() {
        println!("Index = {} Fruit = {:?}", index, i);
        index = index + 1;
    }

    let mut index = 0;
    for i in fruits.keys() {
        println!(
            "Index = {} Key of Fruit = {:?} Fruit = {:?}",
            index,
            i,
            fruits.get(&i)
        );
        index = index + 1;
    }

    let mut new_hash_map = fruits.clone();
    println!("New HashMap = {:?}", new_hash_map);
    new_hash_map.insert(10, String::from("Orange"));
    println!("New HashMap = {:?}", new_hash_map);
    println!("Fruits = {:?}", fruits);
}
