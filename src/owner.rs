/**
 * 所有权
 * 在rust中，每个值有且只有一个所有者变量， 当某个值的所有者离开作用域，该值会被销毁回收。
 * 在理解上述规则需要知道一个信息：
 * rust中对于编译时已知大小存储在栈上的变量在赋值时是会复制一份值，这时有两份值，两个变量各自有其值的所有权；
 * 而对于复杂数据结构（如string)的赋值，其会转移所有权，旧变量也就无法使用；
 *
 *  当只需要使用值而不获取值的所有权时，可以使用引用，有可变引用和不可变引用之分。
 *  不可变引用仅可以读取值，可变引用可以修改值。一个值只能同时有一个可变引用或多个不可变引用，否则会报编译错误。
 *
 * 悬垂引用： 引用指向已回收的内存；编译器在编译时能识别悬垂引用。
 *
 */

pub fn test_owner() {
    let x = 10;
    // scalar变量赋值会被复制，会存在两个10的值，两个变量各自拥有其所有权
    let y = x;
    println!("x = {}; y = {}", x, y);

    let str = String::from("hello");
    // str指向值的所有权限转移给了str1变量， str无法使用
    let str1 = str;
    // 值拷贝了一份，存在两份值，str2拥有新值的所有权
    let str2 = str1.clone();
    // println!("str={}", str); 会报错
    println!("str1 = {} ; str2 = {}", str1, str2);

    // 传参会转移所有权，后面无法使用str2变量，函数的返回值能返回值的所有权
    let (str3, l) = len(str2);
    println!("str3 = {}", str3);

    references();
}

fn len(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

/**
 * 测试引用
 */
fn references() {
    let val = String::from("风急天高猿啸哀");
    // 引用val的值，并不会转移所有权
    let val1 = &val;
    // 可以有多个不可变的引用
    let val2 = &val;
    // val2本来就没有所有权，所以不存在转移所有权的问题
    let val3 = val2;
    println!(
        "val = {}; val1 = {}; val2 = {}; val3 = {};",
        val, val1, val2, val3
    );

    let mut str = String::from("渚清沙白鸟飞回");
    let str2 = &mut str;
    // let str3 = &mut str;
    // 如果不注释上面一行，因为str同时存在两个可变引用，会编译错误
    println!("str2 = {}", str2);

    // let str3 = &str; // 报错， 不能同时存在可变和不可变引用
    // println!("str2 = {}; str3 = {}", str2, str3);

    let mut a1 = String::from("不尽长江滚滚来");
    change(&mut a1);
    println!("a1 = {}", a1);
}

fn change(str: &mut String) {
    str.push_str(",无边落木萧萧下");
}
