use std::io;
fn main(){
    let mut fib = [1, 1, 2];
    
    println!("Select n for which to do the n-th fibbonaci's number");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    
    let n: u32 = n.trim().parse().expect("A number!");
    
    let mut i=2;
    if n==0 {
        println!("The {n}-th fib number is 1!");
        println!("The {n}-th fib number is 1!");
    }
    else if n == 2 {
        println!("The {n}-th fib number is 2!");
    }
    
    while i<n {
        fib[0] = fib[1];
        fib[1] = fib[2];
        fib[2] = fib[0] + fib[1];
        i+=1;
    }
    let number = fib[2];
    println!("The {n}-th fibbonaci number is {number}!");
}