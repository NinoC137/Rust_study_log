fn main() {
    // println!("Hello, world!");
    //
    // another_function(-42, '🍕'); //rust中的函数声明不必须要求在调用行之前
    //
    // let x = plus_one(5);
    //
    // println!("the value of x is: {x}");

    // test_function_about_if_1();
    // test_function_about_if_2();

    // for_loop_test();

    month_loop();
}

// fn another_function(x:i32, unit_label: char) { //rust中建议的命名格式为下划线命名, 而不是驼峰命名
//     println!("Another function. Parameter is {x}, {unit_label}");
// }
//
// fn plus_one(x: i32) -> i32 {
//     // x + 1
//     // 就算不加return, 在statements执行完毕后依旧会返回值
//     // 但是不能加入分号, 否则就会成为Expression, 无返回值
//     return x + 2;
// }

// fn test_function_about_if_1() {
//     let number = 3;
//     if number < 5 {
//         println!("{number} smaller than 5 (condition is true)");
//     }else {
//         println!("{number} bigger than 5 (condition is false)");
//     }
// }

// fn test_function_about_if_2() {
//     let number = 3;
//     if number { //与C语言不同, 这里的变量必须声明为bool变量才可以进if判断
//         println!("{number} was three.");
//     }
// }

// fn for_loop_test() {
//     for number in 1..4 {    //number正向遍历1~4, 进入循环的条件是number小于4
//         println!("{number}!");
//     }
//
//     for number in (1..4).rev() {
//         println!("{number}!");
//     }
//     println!("LIFTOFF!!!");
// }

fn month_loop() {
    //和C语言2维数组类似, 这里存放的是12个字符串element的起始地址
    let months : [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    for index in 0..12 {    //数组index从0开始计数
        println!("current month is: {}", months[index]);
        println!("data address on stack : {:p}", &months[index]);   //栈上的地址
        println!("data address on heap  : {:p}", months[index].as_ptr());   //堆上的地址
    }
}