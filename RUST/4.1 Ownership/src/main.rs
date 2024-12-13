fn main() {
    // String类型
    let mut s = String::from("hello"); // 通过from函数从字符串字面值创建String
    s.push_str(", world!"); // 结尾追加
    println!("The value of s is: {}", s);

    // 移动move
    let s1 = String::from("hello");
    let s2 = s1; // s1的值被移动到s2，s1不再有效
    println!("s1 = {}", s2);

    // 克隆clone
    let s1 = String::from("hello");
    let s2 = s1.clone(); // s1的值被克隆到s2，s1仍然有效
    println!("s1 = {}, s2 = {}", s1, s2);

    // 拷贝copy
    let x = 5;
    let y = x; // x是基本类型，拷贝值到y，x仍然有效
    println!("x = {}, y = {}", x, y);

    // 函数传参（语义类似赋值）
    let s = String::from("hello");
    takes_ownership(s); // s的值被移动到函数内部，在函数结束时被释放，s不再有效
    // println!("s = {}", s); // 报错，s不再有效

    let x = 5;
    makes_copy(x); // x是基本类型，拷贝值到函数内部，x仍然有效
    println!("x = {}", x);

    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    }

    fn makes_copy(some_integer: i32) {
        println!("{}", some_integer);
    }

    // 返回值和作用域
    let s1 = gives_ownership(); // 返回值赋值给s1，s1拥有返回值的所有权

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2的值被移动到函数内部，函数返回值赋值给s3，s3拥有返回值的所有权
    println!("s1 = {}, s3 = {}", s1, s3);

    fn gives_ownership() -> String { // 注："-> String"表示返回值类型
        let some_string = String::from("hello");
        some_string // 返回值移动到调用函数的s1
    }

    fn takes_and_gives_back(a_string: String) -> String {
        a_string // 把参数移动进函数，然后又移动出函数返回（本质和直接移动是一样的，但可以进行更多操作!）
    }
}
