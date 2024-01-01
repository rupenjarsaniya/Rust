// struct Temperature {
//     degree_f: f64,
// }

// impl Temperature {
//     fn freezing() -> Self {
//         Self { degree_f: 32.1 }
//     }

//     fn boiling() -> Self {
//         Self { degree_f: 0.1 }
//     }

//     fn show_temp(&self) {
//         println!("{}", self.degree_f);
//     }
// }

// pub fn main() {
//     let hot = Temperature { degree_f: 10.2 };
//     hot.show_temp();

//     let cold = Temperature::freezing();
//     cold.show_temp();

//     let boiling = Temperature::boiling();
//     boiling.show_temp();
// }

// Activity ----------------------------------------------------------------
enum Color {
    Brown,
    Yellow,
    Green,
    Aqua,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Aqua => println!("Aqua"),
            Color::Brown => println!("Brown"),
            Color::Yellow => println!("Yellow"),
            Color::Green => println!("Green"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("Width: {}", self.width);
        println!("Height : {}", self.height);
        println!("Depth: {}", self.depth);
    }
}

struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}

impl ShippingBox {
    fn new(weight: f64, dimensions: Dimensions, color: Color) -> Self {
        Self {
            weight,
            dimensions,
            color,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("Weight: {}", self.weight);
    }
}

pub fn main() {
    let small_dimensions = Dimensions {
        width: 1.0,
        height: 1.0,
        depth: 1.0,
    };

    let small_box = ShippingBox::new(5.0, small_dimensions, Color::Aqua);
    small_box.print();
}
