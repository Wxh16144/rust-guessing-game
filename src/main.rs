use rand::Rng;
use std::io;

fn main() {
    println!("猜数字");

    println!("请输入一个数字");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("神秘数字是:{}", secret_number);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取行");

    println!("您猜测的数字是:{}", guess);
}
