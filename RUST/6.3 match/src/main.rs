#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 嵌套枚举
}
// match匹配枚举，必须穷举所有可能的值
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // 绑定值也可以提取进行模式匹配
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// 匹配Option<T>类型
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    println!("coin value: {}", value_in_cents(coin));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six: {:?}, none: {:?}", six, none);

    let v = 0u8; // 无符号8位整数，有256种可能
    match v {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        _ => (), // _通配符，匹配所有其他情况
        // ()是空元组，表示什么也不做
    }

    // if let语法糖，只匹配一个模式，简洁
    if let 1 = v {
        println!("one");
    } else { // 可以搭配else使用，匹配其他情况
        println!("not one");
    }
}
