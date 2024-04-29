struct Rectangle {
    width: u32,
    height: u32,
}

// Implementing Method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

impl Rectangle {
    fn can_hold(&self, other_rectangle: &Rectangle) -> bool {
        let area_self = self.width * self.height;
        let area_other = other_rectangle.width * other_rectangle.height;

        if area_self > area_other {
            return true;
        }

        return false;
    }
}

// Associated functions

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width = 30;
    let height = 50;

    println!("The area of the rectangle is {}", area(width, height));

    let rect1 = (width, height);
    println!("The area of the rectangle is {}", area2(rect1));

    let rect2 = Rectangle {
        width: width,
        height: height,
    };

    println!("The area of the rectangle is {}", area3(&rect2));

    // Calling the method
    println!("The area of the rectangle is {}", rect2.area());

    if rect2.width() {
        println!("The rectangle has a nonzero width; it is {}", rect2.width);
    }

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect4 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect5 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect3 hold rect4? {}", rect3.can_hold(&rect4));
    println!("Can rect3 hold rect5? {}", rect3.can_hold(&rect5));

    let sq = Rectangle::square(3);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
