use std::io;

fn main() {
    println!("猜数字");

    println!("请输入一个数字");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取行");

    println!("您猜测的数字是:{}", guess);
}
