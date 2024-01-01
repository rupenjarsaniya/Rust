enum Direction {
    Left,
    Right,
    Up,
    Down,
}

enum Color {
    Red,
    Yellow,
    Green,
    Blue,
}

fn print_color(my_color: Color) {
    match my_color {
        Color::Red => println!("Red"),
        Color::Yellow => println!("Yellow"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}

pub fn main() {
    print_color(Color::Green);
    let go = Direction::Left;
    match go {
        Direction::Left => println!("Left direction"),
        Direction::Right => println!("Right direction"),
        Direction::Up => println!("Up direction"),
        Direction::Down => println!("Down direction"),
    }
}
