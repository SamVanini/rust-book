// Annotation with derived trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Not associated with the type, so not a method
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect1 = (30, 50);

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    // Alternatively, use dbg! (it takes and returns ownership) --> prints to stderr
    // differently from println! (it uses references) --> prints to stdout
    println!("rect2 is {rect2:#?}"); // :? or :#?

    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_variables(width1, height1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_tuples(rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_struct(&rect2)
    );

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect3.area()
    );

    println!("Rect2 can hold Rect3 {}", rect2.can_hold(&rect3));

    let square = Rectangle::square(5);

    println!("Square is {square:#?}")
}

// Problem: is not clear that height and width are related
fn area_with_variables(width: u32, height: u32) -> u32 {
    width * height
}

// Problem. order matters, you do not know which dimension
// is in which position
fn area_with_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
