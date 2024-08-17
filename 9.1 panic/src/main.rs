fn main() {
    // Cargo.toml中panic=abort时，panic!会直接终止且不清理调用栈
    // panic!("crash and burn");

    let v = vec![1, 2, 3];
    v[99]; //越界访问，触发panic
}
