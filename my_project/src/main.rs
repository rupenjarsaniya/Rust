// mod a_data_types;
// mod b_type_casting;
// mod c_operators;
// mod d_if_else;
// mod e_loops;
// mod f_array;
// mod g_slice;
// mod h_tuple;
// mod i_struct;
// mod l_vector;
// mod m_string;
// mod n_hashmap;
// mod o_hashset;
// mod p_iterators;
// mod q_error;
// mod r_enum;
// mod s_ownership;
// mod t_impl;
// mod u_derive;
// mod v_option;
mod x_result;

fn main() {
    // print!("Hello, world! ");
    // print!("I love rust ");

    // println!("Hello, world! ");
    // println!("I love rust ");

    // // Print variables
    // let age = 21;
    // let name = "John";
    // println!("Age: {} & Name: {}", age, name);
    // println!("Here is the newline character\nThis is new line");

    // // Mut keyword to change variable's value
    // let mut x = 21;
    // let y = 1;
    // println!("X = {}", x);
    // println!("Y = {}", y);
    // x = 12;
    // // y = 2;
    // println!("X = {}", x);
    // println!("Y = {}", y);

    // // Constant - Variable name must be uppercase
    // const MY: &str = "Rupen";
    // println!("Name: {}", MY);

    // const PI: f32 = 3.14;
    // println!("PI: {}", PI);

    // let my_name = "Rupen";
    // match my_name {
    //     "Rupen" => println!("that is my name"),
    //     "ABC" => println!("not my name"),
    //     "Any" => println!("hello any"),
    //     _ => println!("nice to meet you"),
    // }

    // Advance match
    // enum Discount {
    //     Percent(i32),
    //     Flat(i32),
    // }

    // struct Ticket {
    //     event: String,
    //     price: i32,
    // }

    // let n = 3;
    // match n {
    //     3 => println!("three"),
    //     other => println!("{} other", other),
    // }

    // let flat = Discount::Flat(2);
    // match flat {
    //     Discount::Flat(2) => println!("flat 2"),
    //     Discount::Flat(amount) => println!("flat: {}", amount),
    //     _ => (),
    // }

    // let concert = Ticket {
    //     event: "concert".to_owned(),
    //     price: 50,
    // };

    // match concert {
    //     Ticket { price: 50, event } => println!("price: 50 event: {}", event),
    //     Ticket { price, .. } => println!("price: {}", price),
    // }

    // a_data_types::main();
    // b_type_casting::main();
    // c_operators::main();
    // d_if_else::main();
    // e_loops::main();
    // f_array::main();
    // g_slice::main();
    // h_tuple::main();
    // i_struct::main();
    // l_vector::main();
    // m_string::main();
    // n_hashmap::main();
    // o_hashset::main();
    // p_iterators::main();
    // q_error::main();
    // r_enum::main();
    // s_ownership::main();
    // t_impl::main();
    // u_derive::main();
    // v_option::main();
    x_result::main();
}

/*
Commnets
Functions
Arithmetic
Basic Math
Control Flow With If & Else
Match
Loop
Enums
Struct
Tuples
Expressions
Ownership
Impl
Vector
String
Derive
Type Annotations
Advance Match
Option
Result
Result and ?
Hashmap
 */
