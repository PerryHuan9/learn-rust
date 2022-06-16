/**
 * 控制流
 *
 * rust中使用if , else if, else 表达式 控制分支，
 *
 * 循环有三种， loop, while, for
 *
 * loop无限循环，可以使用break语句打断循环，多层嵌套时可以指定打断哪一层循环
 *
 */

pub fn test_control_flow() {
    let a = 888;
    if a > 555 {
        println!("a>555={}", true);
    } else if a > 888 {
        println!("a>888={}", true);
    } else {
        println!("a<555={}", true);
    }
    let val = if a > 888 { 888 } else { 666 };
    println!("val={}", val);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let mut x = 1;
    'first: loop {
        let mut y = 0;
        loop {
            println!("x = {}; y = {}", x, y);
            y += 1;
            if (y > 6) {
                x += 1;
            }
            if (x > 6 && y > 6) {
                break 'first;
            }
        }
    }
    let mut idx = 0;
    let res = loop {
        if (idx == 5) {
            // loop 可以有返回值
            break months[idx];
        }
        idx += 1;
    };

    println!("res={}", res);

    let mut index = 0;
    while index < months.len() {
        println!("current mounth is {}", months[index]);
        index += 1;
    }

    for mon in months {
        println!("current mounth = {}", mon);
    }

    println!("fibo5 = {}", print_vec(fifo_vec(5)));
    println!("fibo1 = {}", print_vec(fifo_vec(1)));
    println!("fibo10 = {}", print_vec(fifo_vec(10)));
}

fn fibo(num: u8) -> u32 {
    if num == 0 {
        return 0;
    } else if num == 1 {
        return 1;
    }
    return fibo(num - 2) + fibo(num - 1);
}

fn fifo_vec(num: u8) -> Vec<u32> {
    let mut list = Vec::new();
    let mut index = 0;
    while (index < num) {
        list.push(fibo(index));
        index += 1;
    }
    return list;
}

fn print_vec(ve: Vec<u32>) -> String {
    let mut str = "[ ".to_string();
    for val in &ve {
        str = str + &val.to_string() + ",";
    }
    str += "]";
    return str;
}
