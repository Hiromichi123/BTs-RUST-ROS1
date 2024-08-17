use std::fs::File;
use std::io::Read;

fn main() {
    // 打开不存在的文件
    let f = File::open("hello.txt");
    // 匹配Result两个变体
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() { // 通过error.kind()匹配不同错误类型
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("创建文件失败: {:?}", e),
            },
            // 其他错误类型
            other_error => panic!("打开文件失败: {:?}", other_error),
        },
    };

    // 可简化match嵌套
    // 使用闭包，unwrap_or_else方法
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("创建文件失败: {:?}", error);
            })
        } else {
            panic!("打开文件失败: {:?}", error);
        }
    });

    // unwrap
    let f = File::open("hello.txt").unwrap();
    // expect,在unwrap的基础上自定义错误信息
    let f = File::open("hello.txt").expect("打开文件失败");

    // 传播错误
    let username = read_username_from_file();

    fn read_username_from_file() -> Result<String, std::io::Error> {
        let f = File::open("hello.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
    
        let mut s = String::new();
    
        // match表达式是返回结果
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    // ?运算符简化传播错误
    // 如果Result是Ok，返回Ok中的值
    // 如果Result是Err，将Err的值传播给调用者，相当于return
    let username = read_username_from_file2();

    fn read_username_from_file2() -> Result<String, std::io::Error> {
        let mut f = File::open("hello.txt")?; //若使用了from函数转换错误类型，?会自动调用
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    // ?运算符链式调用
    let username = read_username_from_file3();

    fn read_username_from_file3() -> Result<String, std::io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
    
}

// 可以修改main函数以合法使用？运算符
// 如果函数返回其他类型，可以使用Box<dyn std::error::Error>trait对象作为返回类型
fn main2() -> Result<(), Box<dyn std::error::Error>> { // Box<dyn std::error::Error>可以理解为任何错误类型 
    let f = File::open("hello.txt")?;
    Ok(())
}
