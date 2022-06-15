
/**
 * rust 中变量可以使用let和const关键字进行声明。
 * let声明的变量默认是不可改变的，但是再次声明一个同名变量覆盖它，另外当确实需要改变某个变量时，
 * 可以使用mut关键字修饰，如let mut v1= 888， 这样 v1变量就是可变的了；let变量不能在全局作用域中声明。
 * 
 * const变量永远不可变，可以在任何地方声明，声明时需要指定类型，并且它的值必须在编译时就能计算拿到，
 * 也就是说const变量不能通过let变量计算拿到，或者函数返回。
 * 
 */


const GLOBAL_VAL:u16 = 666;

pub fn test_variable() {
    // let 声明的变量不可改变, let变量不能在全局作用域声明
    let v1 = 999;

    // v1 = 666; error, 不能重新赋值

    // 再次声明一个名字为v1的变量，会覆盖前者
    let v1 = v1 + 1;
    println!("{}", v1); // 1000

    // 新建一个v1变量， mut修饰，它是可变的
    let mut v1 = 666;
    println!("{}", v1);
    v1 += 333;
    println!("改变之后的v1={}", v1); // 999

    // const 声明的变量永远不可变，可以在任何地方声明，并且需要手动指定类型
    const FIRSTDAY: u8 = 1;

    // const 变量必须在编译时就能计算拿到值，如果下面FIRSTDAY换成v1则会报错，因为v1是可变的
    const SECOND: u8 = FIRSTDAY + 1;

    let v2 = 666;
    // const THIRD:u32 = v2 + SECOND; 这样也是不允许的，

    println!("FIRSTDAY={};\nSECOND={};\nGLOBAL_VAL={};", FIRSTDAY, SECOND, GLOBAL_VAL);

}
