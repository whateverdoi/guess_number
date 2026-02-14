use rand::RngExt;
use std::cmp::Ordering;
use std::io;
fn main() {
    let mut count = 0;
    let max_count = 10;
    println!("猜数字游戏,范围为1-100，你有{}次机会", max_count);

    let sercret_number = gen_random_number();
    //把输入的字符串转换成数字

    //比较输入的数字和随机数
    loop {
        let guess = input_guess_number();
        match guess.cmp(&sercret_number) {
            Ordering::Equal => {
                println!("你猜对了！");
                break;
            }
            Ordering::Less => println!("你猜的数字太小了！"),
            Ordering::Greater => println!("你猜的数字太大了！"),
        }
        count += 1;
        if count >= max_count {
            println!("你已经用完了所有机会，游戏结束！");
            break;
        }
        println!("你还有{}次机会", max_count - count);
    }
}

fn gen_random_number() -> u32 {
    //随机数生成
    let sercret_number = rand::rng().random_range(1..=100);
    //println!("随机数是：{}", sercret_number);
    sercret_number
}

fn input_guess_number() -> u32 {
    println!("请输入一个数字：");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("出错了");
    let guess = guess.trim().parse::<u32>().expect("请输入一个数字");
    println!("你猜的数字是：{}", guess);
    guess
}
