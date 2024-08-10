// 计算长方形的面积
#[derive(Debug)] // 为了使用 {:#?} 打印结构体debug
struct Rectangle {
    width: u32,
    length: u32,
}

// 定义一个计算面积的方法
// 注：方法（method）定义该类对象可以执行的行为，不等同于函数
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    // 方法可以有除self外的其他参数
    // 计算长方形是否能够包含另一个长方形
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    // 关联函数，不以self作为参数，类似于静态方法。关联函数通常用于构造器
    // 正方形
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, length: size }
    }
}

// 一个struct可以有多个impl块
impl Rectangle {
    // 一个简单的构造器
    fn new(width: u32, length: u32) -> Rectangle {
        Rectangle { width, length }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, length: 50 };

    println!("{}", rect1.area());
    println!("{:#?}", rect1); // {:#?} 会打印结构体的每个字段, 适合调试

    let rect2 = Rectangle { width: 10, length: 40 };
    let rect3 = Rectangle { width: 60, length: 45 };

    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));

    let s = Rectangle::square(20);
}


