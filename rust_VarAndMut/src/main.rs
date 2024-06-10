fn main() {
    const THREE_HOUR_IN_SECONDS : u32 = 60 * 60 * 3;    //const常量

    /* shadowing机制与作用域机制 */

    println!("const of THREE_HOUR_IN_SECONDS is {THREE_HOUR_IN_SECONDS}");

    let x = 5;
    let x = x + 1;

    {   //内部作用域的变量属性单独分配
        let x = x * 2;
        println!("the value of x in the inner scope is: {x}");
    }

    println!("the value of x is: {x}");

    /*
        如果我们想要得到spaces字符串变量的长度, 我们同样需要用到 shadowing机制
        如果用C语言的思维, 可能会想要这样做:
            let mut spaces = "    ";
            spaces = spaces.len();
        这会导致报错"变量类型不一致", 可以理解为我们先把space变量声明为了字符串变量,
        但是在后续的变量赋值中, 将其赋值为整数变量, 导致冲突.
    */
    let spaces = "    ";
    let spaces = spaces.len();

    println!("length of spaces is: {spaces}");
}
