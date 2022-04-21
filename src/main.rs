use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("猜数字");

    println!("请输入一个数字");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("神秘数字是:{}", secret_number);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取行");

    let guess:u32 = guess.trim().parse().expect("请输入数字");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("太小了"),
        Ordering::Greater => println!("太大了"),
        Ordering::Equal => println!("猜对了"),
    }

    println!("您猜测的数字是:{}", guess);
}
