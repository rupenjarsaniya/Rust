pub fn main() {
    println!("------------------\nARRAY\n------------------");

    let numbers: [i8; 6] = [1, 2, 3, 4, 5, 'a' as i8];
    println!("Numbers = {:?}", numbers);

    let characters: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    println!("Characters = {:?}", characters);

    // Array with default values
    let new_arr: [i32; 5] = [3; 5];
    println!("Arrays = {:?}", new_arr);

    println!("Array 1 = {}", characters[1]);

    let arr_len = new_arr.len();
    println!("Length of array = {}", arr_len);

    let slice_arr = &characters;
    let slice_arr1 = &characters[0..2];
    println!("Array slice = {:?}", slice_arr);
    println!("Array slice 1 = {:?}", slice_arr1);
}
