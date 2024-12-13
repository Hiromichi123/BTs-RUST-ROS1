fn main() {
    // 实现类似引用传递（获取值但不转移所有权）
    let s1 = String::from("hello");

    let (len, s1) = calculate_length(s1);
    // 获得了s1的长度和s1的所有权

    println!("The length of '{}' is {}.", s1, len);

    fn calculate_length(s: String) -> (usize, String) {
        let length = s.len(); // 返回长度
        (length, s)
    }

    // 引用传递，默认不可变引用，加mut可变引用
    let mut s1 = String::from("hello");

    let len = calculate_length2(&mut s1);

    println!("The length of '{}' is {}.", s1, len);

    fn calculate_length2(s: &mut String) -> usize {
        s.push_str(", world");
        s.len()
    }

    //可以通过创建一个新作用域，允许拥有多个可变引用
    let mut s1 = String::from("hello");
    {
        let r1 = &mut s1;
        println!("{}", r1);
    }
    let r2 = &mut s1;
    println!("{}", r2);

    //不能在拥有不可变引用的同时拥有可变引用（同一作用域下）
    let mut s1 = String::from("hello");
    s1.push_str(", world");
    let r1 = &s1;
    let r2 = &s1;
    // let r3 = &mut s1; // 报错
    println!("{}, {}", r1, r2);

    //悬空引用
    let r = dangle();
    println!("{}", r);

    fn dangle() -> String {
        let s = String::from("hello");
        s
      //&s  引用在出函数作用域后被释放，悬空引用会导致Rust编译器报错
    }
}
