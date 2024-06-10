# Rust中的function函数

## Functions

You’ve also seen the `fn` keyword, which allows you to declare new functions.

Filename: src/main.rs

```rust
fn main() {
    println!("Hello, world!");

    another_function(); //rust中的函数声明不必须要求在调用行之前
}

fn another_function() { //rust中建议的命名格式为下划线命名, 而不是驼峰命名
    println!("Another function.");
}
```



## Parameters

The first parameter is named `x` and is an `i32`. 

The second is named `unit_label` and is type `char`. 

The function then prints text containing both the `x` and the `unit_label`.

Filename: src/main.rs

```rust
fn main() {
    println!("Hello, world!");

    another_function(-42, '🍕'); //rust中的函数声明不必须要求在调用行之前
}

fn another_function(x:i32, unit_label: char) { //rust中建议的命名格式为下划线命名, 而不是驼峰命名
    println!("Another function. Parameter is {x}, {unit_label}");
}
```

Try running this program; you should get the following output:

```shell
$ cargo run
   Compiling rust_Functions v0.1.0 (/home/nino/RustStudy/rust_Functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/rust_Functions`
Hello, world!
Another function. Parameter is -42, 🍕
```



## Statements and Expressions

### statements

Function bodies are made up of a series of statements optionally ending in an expression. 

So far, the functions we’ve covered haven’t included an ending expression, but you have seen an expression as part of a statement. Because Rust is an expression-based language, this is an important distinction to understand. Other languages don’t have the same distinctions, so let’s look at what statements and expressions are and how their differences affect the bodies of functions.

- **Statements** are instructions that perform some action and **do not return a value**.
- **Expressions** evaluate to **a resultant value**. Let’s look at some examples.

We’ve actually already used statements and expressions. Creating a variable and assigning a value to it with the `let` keyword is a statement. In follwing, `let y = 6;` is a statement.

Filename: src/main.rs

```rust
fn main() {
    let y = 6;
}
```

### Expressions

A new scope block created with curly brackets is an expression, for example:

Filename: src/main.rs

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

This expression:

```rust
{
    let x = 3;
    x + 1
}
```

is a block that, in this case, **evaluates to `4`. That value gets bound to `y`** as part of the `let` statement. 

Note that the `x + 1` line **doesn’t have a semicolon at the end**, which is unlike most of the lines you’ve seen so far. 

**Expressions do not include ending semicolons.** 

**If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.** 

Keep this in mind as you explore function return values and expressions next.



## Functions with Return Values

Functions can return values to the code that calls them. 

We don’t name return values, but we must **declare their type after an arrow** (`->`). In Rust, the return value of the function is synonymous with the value of the **final expression in the block of the body of a function**. 

You can return early from a function by using the `return` keyword and specifying a value, but most functions return the last expression implicitly. Here’s an example of a function that returns a value:

Filename: src/main.rs

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 //就算不加return, 在statements执行完毕后依旧会返回值
    //return x + 2;
}
```

There are no function calls, macros, or even `let` statements in the `plus_one` function. 

That’s a perfectly valid function in Rust. Note that the function’s return type is specified too, as `-> i32`. Try running this code; the output should look like this:

```console
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/functions`
The value of x is: 6
```

The 6 in `plus_one` is the function’s return value, which is why the return type is `i32`. 
There are two important bits: 

​	First, the line `let x = plus_one();` shows that we’re using the return value of a function to initialize a variable. Because the function `plus_one` returns a `6`, that line is the same as the following:

```rust
let x = 5 + 1;
```

​	Second, the `plus_one` function has a parameters and defines the type of the return value, but the body of the function is a lonely `x + 1` with no semicolon because it’s an expression whose value we want to return.

​	

## If 条件语句

An `if` expression allows you to branch your code depending on conditions.

You provide a condition and then state.

 “If this condition is met, run this block of code. If the condition is not met, do not run this block of code.”

Filename: src/main.rs

```rust
fn main() {
    let number = 3;
    if number < 5 {
        println!("{number} smaller than 5 (condition is true)");
    }else {
        println!("{number} bigger than 5 (condition is false)");
    }
}
```

output:

``` shell
$ cargo run
   Compiling rust_Functions v0.1.0 (/home/nino/RustStudy/rust_Functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/rust_Functions`
3 smaller than 5 (condition is true)
```



Filename: src/main.rs

``` rust
fn test_function_about_if_2() {
    let number = 3;
    if number { //与C语言不同, 这里的变量必须声明为bool变量才可以进if判断
        println!("{number} was three.");
    }
}
```

output:

``` shell
$ cargo run
   Compiling rust_Functions v0.1.0 (/home/nino/RustStudy/rust_Functions)
error[E0308]: mismatched types
  --> src/main.rs:36:8
   |
36 |     if number {
   |        ^^^^^^ expected `bool`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rust_Functions` (bin "rust_Functions") due to 1 previous error
```



If we want the `if` code block to run only when a number is not equal to `0`, for example, we can change the `if` expression to the following:

Filename: src/main.rs

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```

Running this code will print `number was something other than zero`.



### handling multiple conditions with `else if`

You can use multiple conditions by combining `if` and `else` in an `else if` expression. For example:

Filename: src/main.rs

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

This program has four possible paths it can take. After running it, you should see the following output:

```shell
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
number is divisible by 3
```

Using too many `else if` experssions can clutter your code, so if you have more than one, you'd better use `match` to refactor your code.



### Using if in a let statement

Because `if` is an expression, we can use it on the right side of a `let` statement to assign the outcome to a variable.

Filename: src/main.rs

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 }; //没有分号, 所以是statement, 有返回值
	//注意: 这里的if返回值数据类型也要与左值匹配, 否则编译时会报错
    println!("The value of number is: {number}");
}
```

**Assigning the result of an `if` expression to a variable**

The `number` variable will be bound to a value based on the outcome of the `if` expression. Run this code to see what happens:

```shell
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/branches`
The value of number is: 5
```



## Loop循环语句

### Returning Value from Loops

One of the uses of a `loop` is to retry an operation you know might fail, such as checking whether a thread has completed its job. You might also need to pass the result of that operation out of the loop to the rest of your code. 

To do this, you can add the value you want returned after the `break` expression you use to stop the loop; that value will be returned out of the loop so you can use it, as shown here:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

Before the loop, we declare a variable named `counter` and initialize it to `0`. 

Then we declare a variable named `result` to hold the value returned from the loop. 

On every iteration of the loop, we add `1` to the `counter` variable, and then check whether the `counter` is equal to `10`. When it is, we use the `break` keyword with the value `counter * 2`. After the loop, we use a semicolon to end the statement that assigns the value to `result`. Finally, we print the value in `result`, which in this case is `20`.



### Loop Labels to Disambiguate Between Multiple Loops

If you have loops within loops, `break` and `continue` apply to the innermost loop at that point.

You can optionally specify a ***loop label*** on a loop that you can then use with `break` or `continue` to specify that those keywords apply to the labeled loop instead of the innermost loop.

Loop labels must begin with a single quote`'`. Here’s an example with two nested loops:

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

The outer loop has the label `'counting_up`, and it will count up from 0 to 2. The inner loop without a label counts down from 10 to 9. The first `break` that doesn’t specify a label will exit the inner loop only. The `break 'counting_up;` statement will exit the outer loop. This code prints:

```shell
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.58s
     Running `target/debug/loops`
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
```



## Conditional Loops with `while`

A program will often need to evaluate a condition within a loop. While the condition is `true`, the loop runs. When the condition ceases to be `true`, the program calls `break`, stopping the loop. 

It’s possible to implement behavior like this using a combination of `loop`, `if`, `else`, and `break`; 

we use `while` to loop the program three times, counting down each time, and then, after the loop, print a message and exit.

Filename: src/main.rs

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

**Using a `while` loop to run code while a condition holds true**

While a condition evaluates to `true`, the code runs; otherwise, it exits the loop.



## Looping Through a Collection with `for`

You can choose to use the `while` construct to loop over the elements of a collection, such as an array. 

For example, prints each element in the array `a`.

Filename: src/main.rs

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

**Looping through each element of a collection using a `while` loop**

Running this code will print every element in the array:

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/loops`
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
```

As a more concise alternative, you can use a `for` loop and execute some code for each item in a collection. A `for` loop looks like the code.

Filename: src/main.rs

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

**Looping through each element of a collection using a `for` loop.**

Using the `for` loop, you wouldn’t need to remember to change any other code if you changed the number of values in the array.

Most Rustaceans would use a `for` loop. The way to do that would be to use a `Range`, provided by the standard library, which generates all numbers in sequence starting from one number and ending before another number.

Here’s what the countdown would look like using a `for` loop and another method we’ve not yet talked about, `rev`, to reverse the range:

Filename: src/main.rs

```rust
fn main() {
    for number in 1..4 {    //number正向遍历1~4, 进入循环的条件是number小于4
        println!("{number}!");
    }
    
    for number in (1..4).rev() { //number反向遍历1~4, 进入循环的条件是number小于4
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```



## Summary

结合`数组`, `for`, `迭代器`等知识点, 实现一个简单的字符串数组打印

Program:

``` rust
fn main() {
    //和C语言2维数组类似, 这里存放的是12个字符串element的起始地址
    let months : [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    for index in 0..12 {    //数组index从0开始计数
        println!("current month is: {}", months[index]);
        println!("data address on stack : {:p}", &months[index]);   //栈上的地址
        println!("data address on heap  : {:p}", months[index].as_ptr());   //堆上的地址
    }
}
```

Output:

``` shell
$ cargo run
   Compiling rust_Functions v0.1.0 (/home/nino/RustStudy/rust_Functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/rust_Functions`
current month is: January
data address on stack : 0x7ffe67535c90
data address on heap  : 0x5d4bf555806b
current month is: February
data address on stack : 0x7ffe67535ca0
data address on heap  : 0x5d4bf5558072
current month is: March
data address on stack : 0x7ffe67535cb0
data address on heap  : 0x5d4bf555808a
current month is: April
data address on stack : 0x7ffe67535cc0
data address on heap  : 0x5d4bf555808f
current month is: May
data address on stack : 0x7ffe67535cd0
data address on heap  : 0x5d4bf5558094
current month is: June
data address on stack : 0x7ffe67535ce0
data address on heap  : 0x5d4bf5558097
current month is: July
data address on stack : 0x7ffe67535cf0
data address on heap  : 0x5d4bf555809b
current month is: August
data address on stack : 0x7ffe67535d00
data address on heap  : 0x5d4bf555809f
current month is: September
data address on stack : 0x7ffe67535d10
data address on heap  : 0x5d4bf55580a5
current month is: October
data address on stack : 0x7ffe67535d20
data address on heap  : 0x5d4bf55580ae
current month is: November
data address on stack : 0x7ffe67535d30
data address on heap  : 0x5d4bf555807a
current month is: December
data address on stack : 0x7ffe67535d40
data address on heap  : 0x5d4bf5558082
```

​	可以看出, 对于保存在栈区中的数组元素中的"保存着字符串头地址的指针变量",  都是每次按4 Bytes进行增长, 对应了一个指针变量的大小, 其中指针变量的值即为指向堆区的地址, 这个地址即为字符串头的地址.

​	在堆区的地址中, 字符串变量之间的地址也是连续的, 并且地址每次增加的长度也与字符串的长度所对应.