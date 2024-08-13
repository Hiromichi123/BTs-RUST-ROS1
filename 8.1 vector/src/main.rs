use std::vec;

fn main() {
    // 使用Vec::new()创建一个空的vector
    let v: Vec<i32> = Vec::new(); // rust需要显式声明类型，因为vector在heap上分配内存

    // 使用宏vec!创建带初始值的vector
    let mut v = vec![1, 2, 3]; // rust会自动类型推导

    // 更新vector中的元素
    v.push(1); //末尾添加一个新元素
    v.pop(); // 删除最后一个元素

    // 使用索引访问元素
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

     // 使用get方法访问元素
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 遍历vector中的元素
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v { // 每次迭代，i会被绑定到v中的一个元素
        *i += 50; // *i表示解引用，获取i指向的值
        println!("{}", i);
    }

    // vector和enum的结合使用
    // 利用可附加数据类型的enum特性
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // 创建一个可以存放不同类型的值的vector
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    //注：在不知道或不详尽vector存储的元素类型时，用trait对象Box<dyn Trait>来存储不同类型的值
}
