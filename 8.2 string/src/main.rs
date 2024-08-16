fn main() {
    // 类似Vec<T>的操作String::new()创建一个空的String
    let mut s = String::new();

    // to_string()方法创建带初始值的String
    let data = "initial contents";
    let s: String = data.to_string(); // 等价于let s = String::from(data);
    println!("{}", s);

    // 更新String
    let mut s = String::from("foo");
    s.push_str("bar"); // push_str()方法附加字符串字面值
    println!("{}", s);

    let mut s = String::from("lo");
    s.push('l'); // push()方法附加单个字符
    println!("{}", s);

    // + 拼接 
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1的所有权被转移，不能再使用
    println!("{}", s3);

    // format!宏拼接
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    // 索引
    let s = String::from("hello");
    let h = &s[0..1]; // 索引必须是有效的UTF-8字符边界

    // 三种遍历方法：字节、Unicode标量值、字形簇（标准库未提供）
    let w = String::from("नमस्ते");
    // 以字节为单位遍历
    for b in w.bytes() {
        println!("{}", b);
    }
    // 以Unicode标量值为单位遍历
    for c in w.chars() {
        println!("{}", c);
    }
    // 注：中文在UTF-8中占3个字节，但是在Unicode中是一个字符


}
