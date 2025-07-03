struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self{
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 3,
        height: 6,
    };

    let rect2 = Rectangle{
        width: 30,
        height: 12,
    };

    let sq = Rectangle::square(69);

    println!("Rectangle 1 area is {}", rect1.area());
    println!("Rectangle 2 area is {}", rect2.area());

    println!("Square area is {}", sq.area());

    println!("Can rect 1 hold rect 2? {}", rect1.can_hold(&rect2));
    println!("Can rect 2 hold rect 1? {}", rect2.can_hold(&rect1));
}
