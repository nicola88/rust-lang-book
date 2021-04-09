#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height  
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    println!("rect1 = {:#?}", rect1);
    println!("The area of the rectangle is {}", rect1.area());
    let rect2 = Rectangle {
        width: 10,
        height: 40
    };
    println!("rect2 = {:#?}", rect2);
    println!("The area of the rectangle is {}", rect2.area());
    let rect3 = Rectangle {
        width: 60,
        height: 45
    };
    println!("rect3 = {:#?}", rect3);
    println!("The area of the rectangle is {}", rect3.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    let sq1 = Rectangle::square(25);
    println!("sq1 = {:#?}", sq1);
    println!("The area of the square is {}", sq1.area());
}
