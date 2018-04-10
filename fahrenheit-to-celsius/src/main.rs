use std::io;

fn main() {
    println!("输入华氏温度");
    let mut fahrenheit = String::new();
    io::stdin().read_line(&mut fahrenheit)
        .expect("Failed to read line");
    let fahrenheit: f64 = fahrenheit.trim().parse()
        .expect("需要输入一个数字");
    println!("输入的华氏温度为： {}", fahrenheit);
    let celsius = (fahrenheit - 32.0) * 5.0/9.0;
    println!("转换后的摄氏温度为： {}", celsius);
}