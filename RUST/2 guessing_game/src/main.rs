use std::io; // io库属于 prelude(预导入模块)
use rand::Rng; // trait(接口), 它定义了随机数生成器的一些方法
use std::cmp::Ordering; // 枚举类型, 它有三个值: Less, Greater, Equal

fn main() {
    println!("猜数!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    //随机数生成器位于本地线程空间，并通过操作系统获取随机数种子
    println!("秘密数字是: {}", secret_number);

    loop{
        println!("猜测一个数:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行");
        // stdin().readline()其实有返回值io::Result, 它是一个枚举类型 Ok 或 Err

        println!("你猜测的数是: {}", guess); // {}是占位符

        //shadow隐藏同名旧变量，通常用在需要类型转换的场景
        // 左侧显示声明变量类型
        // trim()去掉字符串首尾的空白字符, parse()将字符串转换为数字
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _是通配符, 匹配所有Err
        };

        match guess.cmp(&secret_number){ // 三个分支(arm)
            Ordering::Less => println!("太小了!"),
            Ordering::Greater => println!("太大了!"),
            Ordering::Equal => {
                println!("你赢了!");
                break;
            }
        }
    }
    
}
