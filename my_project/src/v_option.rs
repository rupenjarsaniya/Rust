struct Customer {
    age: Option<i32>,
    email: String,
}
pub fn main() {
    let users = vec![
        Customer {
            age: Some(23),
            email: "test@example.com".to_owned(),
        },
        Customer {
            age: None,
            email: "test@example.com".to_owned(),
        },
    ];

    for user in users {
        match user.age {
            Some(age) => println!("{}", age),
            None => println!("No age specified"),
        }
    }
}
