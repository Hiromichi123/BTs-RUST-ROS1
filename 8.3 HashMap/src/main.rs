// 由于不常用，HashMap没有prelude
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // insert()方法插入键值对
    scores.insert(String::from("Blue"), 10);

    // 使用collect方法将元组的vector转换为HashMap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // zip方法创建一个元组的vector
    let scores: HashMap<_, _> = 
        teams.iter().zip(initial_scores.iter()).collect();
        // zip有拉链的意思，将两个迭代器的元素一一对应组合成元组
        // collect方法将元组的vector转换为HashMap，HashMap<_, _>是类型推断（由于collect方法会收集多种类型，所以需要类型推断）

    // HashMap所有权
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // insert方法会获取所有权，如果插入引用，就不会获取所有权
    map.insert(&field_name, &field_value);
    // 直接插入会造成所有权的转移
    // map.insert(field_name, field_value); // HashMap的同构性，只能存储相同类型，所以这里会报错
    // 注：通常直接插入值而不是引用以减少生命周期问题，另外可以通过clone方法解决

    // 访问HashMap的值
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {  // get方法返回一个Option<&V>，需要进行match处理
        Some(v) => println!("The score of Blue is {}", v),
        None => println!("No score for Blue"),
    }

    // 遍历HashMap
    for (key, value) in &scores {
        // 通常遍历后还需要使用HashMap，所以使用引用来遍历
        println!("{}: {}", key, value);
    }
}
