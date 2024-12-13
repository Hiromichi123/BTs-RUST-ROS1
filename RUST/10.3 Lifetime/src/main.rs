fn main() {
    let string1 = String::from("abcd"); //string1 生命周期2-8行
    {
        let string2 = "xyz"; // string2 是字面值，整个程序运行期间都有效
        let result = longest(string1.as_str(), string2); // result 生命周期取参数中最短string1，是2-8行
        println!("The longest string is {}", result);
    }

    // 因为不知道传入参数的生命周期，需要手动指定生命周期参数
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // struct中的生命周期标注 
    struct ImportantExcerpt<'a> {
        part: &'a str, // 指定了part的生命周期要比ImportantExcerpt的生命周期长
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt { // i的生命周期25-结尾
        part: first_sentence // first_sentence的生命周期23-结尾
    };

    // 生命周期省略规则
    // 1. 每个引用的参数都有自己的生命周期参数
    // 2. 如果只有一个输入生命周期参数，那么它被赋给所有输出生命周期参数
    // 3. 如果有多个输入生命周期参数，其中一个是&self或&mut self，那么self的生命周期被赋给所有输出生命周期参数

    // 生命周期省略规则的例子
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }


    // 方法中的生命周期标注
    //impl<'a>表示可以使用生命周期为'a的引用，ImportantExcerpt以及self的生命周期都是'a
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }

    // 静态生命周期
    let s: &'static str = "I have a static lifetime.";
    // 'static生命周期是整个程序运行期间有效的，比如所有的字符串字面值都是'static生命周期
}


