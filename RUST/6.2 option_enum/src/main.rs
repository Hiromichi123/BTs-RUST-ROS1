// Option<T>是一个泛型枚举，它有两个成员：Some和None
// 定义在标准库中，直接使用即可
enum Option<T> {
    Some(T), // Some成员保存了一个值，这个值的类型是T(泛型)
    None,
}

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None; // 标准库中提供的和自己写的Option冲突，可以无视报错
}
