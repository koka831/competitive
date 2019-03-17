

fn main() {
    assert_eq!(6, fact(3));
    println!("{}", fib(40));
}

fn fact(n: usize) -> usize {
    if n == 0 { return 1; }
    n * fact(n - 1)
}

fn fib(n: usize) -> usize {
    if n <= 1 { return n; }
    fib(n - 1) + fib(n - 2)
}
