use std::io;

fn main() {
    println!("输入要求的N阶fib数列");
    let mut num = String::new();

    io::stdin().read_line(&mut num)
        .expect("Failed to read line");

    let num: i32 = num.trim().parse()
        .expect("需要输入一个数字");

    println!("输入的N为：{}", num);

    let total = fib(num);

    println!("结果为：{}", total);

}


fn fib(fib_num:i32) -> u64 {
    if fib_num==1 { 1 }
    else if fib_num==2 { 1 }
    else { fib(fib_num-1) + fib(fib_num-2) }
}
