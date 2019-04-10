use std::io;

fn main() {

    let mut fib: Vec<u64> = Vec::new();

    fn compute_fib(n: u32, fib: &mut Vec<u64>) -> u64 {
        for i in fib.len()..n {

        }
        fib[n]
    }

    loop {
        println!("Which Fibonacci number do you want?");
        let mut number = String::new();

        io::stdin().read_line(&mut number)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}
