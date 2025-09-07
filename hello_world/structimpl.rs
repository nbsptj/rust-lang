fn main() {
    let rect1 = Rectange {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);

    println!("The area of rectange is {} square pixels.", rect1.area());

    let rect2 = Rectange {
        width: 20,
        height: 40,
    };

    let rect3 = Rectange {
        width : 35,
        height: 60,
    };

    println!("rect1 is can_hold rect2 {}", rect1.can_hold(&rect2));
    println!("rect1 is can_hold rect3 {}", rect1.can_hold(&rect3));

    let square = Rectange::square(10);
    println!("the square area is {}", square.area());
}

#[derive(Debug)]
struct Rectange {
    width: u32,
    height: u32,
}

impl Rectange {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectange) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Rectange {
        Rectange {
            width: size,
            height: size,
        }
    }
}