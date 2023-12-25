use std::io;
fn main() {
    loop{
    println!("Enter the value n: ");
    let mut num=String::new();
    io::stdin().read_line(&mut num)
        .expect("Enter right value");
    let mut num:u32=num.trim().parse()
        .expect("Enter a number");
    num=fibonacci(num);
    println!("Fibbonacy number: {}",num);
    
    }
}
fn fibonacci(n:u32) ->u32{
    let mut a=1;
    let mut b=0;
    let mut i=1;
    let mut fib=0;
    while i<n{
        b=a;
        a=fib;
        fib=a+b;
        i=i+1;
    }
    fib
}
