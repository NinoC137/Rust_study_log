# 第一个Rust程序 -- 由cargo构建

## What is "Cargo"

​	**cargo is Rust's build system and package manager**

​	cargo can handles a lot of tasks for you, such as building your code, downloading the libraries your codes depends on, and building those libraries.

## Create a Project with cargo

 	we can create a workspace by cargo, run the following:

``` shell
$ cargo new hello_rust
$ cd hello_rust
```

​	The first command creates a new directory and project named *hello_rust*.

​	Go into the *hello_rust* directory and list the files, we'll see that cargo has generated two files and one directory for us: a *Cargo.toml* file and a *src* directory with a *main.rs* file inside.

​	Also, we can create a project with VSC tool like Git. we can using `cargo new --vsc=git` to create it.

## Build your Project

​	From your *hello_rust* directory, build your project by entering the following command:

``` shell
$ cargo build
   Compiling hello_rust v0.1.0 (/home/nino/RustStudy/hello_rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.47s
```

​	After building, we'll get an executable file in *target/debug/hello_rust*. Because the default building is a debug build, Cargo puts the binary in a directory named *debug*. We can run the executable file with this command.

``` shell
$ ./target/debug/hello_rust 
Hello, world!
```

​	We just built a project with `cargo build` and ran it with `./target/debug/hello_rust`, but we can also use `cargo run` to compile the code and then run the resultant executable all in one command:

``` shell
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello_rust`
Hello, world!
```

​	Actually, use `cargo run` is better for most developer.



## Recap all we learned

Let’s recap what we’ve learned so far about Cargo:

- We can create a project using .`cargo new`
- We can build a project using .`cargo build`
- We can build and run a project in one step using .`cargo run`
- We can build a project without producing a binary to check for errors using .`cargo check`
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the *target/debug* directory.



## Building for Release

​	When your project is finally ready for release, you can use `cargo build --release`. 

​	The optimizations make your Rust code run faster, bug turning them on lengthens the time it takes for  your program to compile.