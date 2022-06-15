use ferris_says::say;
use rand::Rng;
use std::{cmp::Ordering, io};

pub fn do_guess() {
    ferris_say();
    let number = rand::thread_rng().gen_range(0..101);
    loop {
        println!("请输入你要猜的数字：");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取输入失败");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&number) {
            Ordering::Greater => println!("大了未必佳"),
            Ordering::Less => println!("小，未必就是珍珠"),
            Ordering::Equal => {
                println!("众里寻他千百度，蓦然回首，那人却在灯火阑珊处");
                break;
            }
        }
    }
}

fn ferris_say() {
    let msg = String::from("猜猜看");
    let width = msg.chars().count();
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());
    say(msg.as_bytes(), width, &mut writer);
}




