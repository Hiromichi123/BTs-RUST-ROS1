fn main() {
    // 变量, 默认是不可变的
    let x = 5; // x is immutable
    print!("The value of x is: {}\n", x);
    let mut y = 5; // y is mutable
    y += 1;
    print!("The value of y is: {}\n", y);

    // String类型
    let s = String::from("hello"); // 通过from函数从字符串字面值创建String
    s.push_str(", world!"); // 结尾追加
    print!("The value of s is: {}\n", s);

    // 常量
    const MAX_POINTS: u32 = 100_000;
    print!("The value of MAX_POINTS is: {}\n", MAX_POINTS);

    // shadowing
    // 用法1
    let x = 6; // shaoowing之前x为5，接下来x为6
    print!("The value of x is: {}\n", x);

    // 用法2
    let space = "    ";
    let space = space.len(); // space为4， 快捷方便
    print!("The value of space is: {}\n", space);

    // 函数
    fn five(x: i32) -> i32 {
        x + 5
    }
    // call five
    let x = five(6); // 5 + 6
    println!("The value of x is: {}\n", x);

    // 控制流
    // if
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if 作为表达式
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}\n", number);

    // loop 循环, 可作为语句或者表达式
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // break返回值
        }
    };
    println!("The result is: {}\n", result);

    // while 循环
    let mut number = 3;
    while number != 0 {
        number = number - 1;
    }
    println!("LIFTOFF!!!\n");

    // for 循环
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() { // a.iter()返回一个迭代器
        println!("the value is: {}", element);
    }

    // range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!\n");
}
