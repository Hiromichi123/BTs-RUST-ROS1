fn main() {
    // 类引用传递
    let s1 = String::from("hello");

    let (len, s1) = calculate_length(s1);
    // 获得了s1的长度和s1的所有权

    println!("The length of '{}' is {}.", s1, len);

    fn calculate_length(s: String) -> (usize, String) {
        let length = s.len(); // 返回长度
        (length, s)
    }
}
