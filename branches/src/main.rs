fn main() {
    let number = 3;

    if number < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    };

    let value = fib(3);
    println!("fib number 3 is {value}");

    let value = fib(5);
    println!("fib number 5 is {value}");
}

// 0, 1, 1, 2, 3, 5, 8, 13...
fn fib(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
