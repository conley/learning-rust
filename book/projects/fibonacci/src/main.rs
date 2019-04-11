use std::io;

fn main() {
    let mut fib: Vec<usize> = vec![0];

    loop {
        println!("Which Fibonacci number do you want?");
        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read input");

        let number: usize = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("fibonacci({}) = {}", number, compute_fib(number, &mut fib));
    }
}

fn compute_fib(n: usize, fib: &mut Vec<usize>) -> usize {
    for i in fib.len()..n + 1 {
        let last = fib[i - 1];
        fib.push(last + i)
    }
    fib[n]
}
