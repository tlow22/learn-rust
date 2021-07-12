#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        return self.width * self.height
    }

    fn can_fit(&self, other: &Rectangle) -> bool {
        return self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };

    let rect2 = Rectangle {
        width: 5,
        height: 10,
    };

    let rect3 = Rectangle {
        width: 30,
        height: 10,
    };

    let rect4 = Rectangle {
        width: 20,
        height: 10,
    };
    
    // print out rectangle info
    println!("rect is {:?}", rect);
    println!("rect area is {}", rect.area());
    
    // print if rect can fit rect2, rect3, rect4
    println!("rect can fit rect2? {}", rect.can_fit(&rect2));
    println!("rect can fit rect3? {}", rect.can_fit(&rect3));
    println!("rect can fit rect4? {}", rect.can_fit(&rect4));
}

