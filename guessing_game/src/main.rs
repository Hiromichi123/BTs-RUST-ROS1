use std::io; //prelude预导入模块

fn main() {
    println!("猜数!");

    println!("猜测一个数:");

    let foo = 1;
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取行");

    println!("你猜测的数是: {}", guess);
}
