use std::io;

fn main() {
    loop {
        println!("Enter a temperature in degrees");
        let mut temp = String::new();
        io::stdin().read_line(&mut temp)
            .expect("Failed to read input.");
        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Could not parse {} as a number: {}", temp.trim(), e);
                continue
            },
        };
        println!("Is that in Celsius? [y/n]");
        let mut response = String::new();
        io::stdin().read_line(&mut response)
            .expect("Failed to read input.");
        let is_celsius: bool = response.trim().to_lowercase().starts_with('y');
        if is_celsius {
            println!("{} C ==> {} F", temp, temp * 9.0 / 5.0 + 32.);
        } else {
            println!("{} F ==> {} C", temp, (temp - 32.) * 5.0 / 9.0);
        }
    }
}
