// 编写一个函数：
// 1. 接收字符串作为参数
// 2. 返回字符串里的第一个单词
// 3. 如果函数没有找到空格，返回整个字符串

// 注：参数传入字符串切片而不是String，这样函数可以接收String和&str，API更加通用
fn first_word(s: &str) -> &str { // 返回字符串切片
    let bytes = s.as_bytes(); // 将字符串转换为字节数组
    for (i, &item) in bytes.iter().enumerate() { // enumerate() 方法返回一个元组，包含索引和元素
        if item == b' ' { // 匹配， 其中b表示字节字面量
            return &s[..i]; // 返回字符串的切片
        }
    }
    &s[..] // 没有空格，返回整个字符串切片
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // 对s的不可变借用
    println!("The first word is: {}", word);
    // 此时word不再被使用，对s的不可变借用结束
    s.clear(); // 清空字符串，对s的可变借用
    //println!("The first word is: {}", word); // s清空，word的引用已经失效，编译报错



    // 字符串切片
    let s = String::from("hello world");
    let hello = &s[0..5]; // 等价于&[..5]
    let world = &s[6..11]; // 等价于&[6..len] 或 &[6..]
    let slice = &s[..]; // 等价于&[0..len]
    println!("hello: {}, world: {}, slice: {}", hello, world, slice);

    // 其他类型的切片（数组为例），切片本质上是引用集合的一部分
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // 存储了一个指针指向数组的开始位置，外加一个长度
    println!("slice: {:?}", slice);
} 
