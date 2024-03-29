use std::io;
use rand::Rng;


fn main() {

    println!("猜猜数字!");

    let select_number = rand::thread_rng().gen_range(1..=100);

    println!("神秘的密钥{select_number}");

    println!("请输入一个数字");

    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("读取错误");


    println!("你输入的数字是: {}", number);
}
