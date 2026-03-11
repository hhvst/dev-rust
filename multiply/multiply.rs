use std::io;

fn main() {
    let mut x = String::new();
    let mut counter: u32 = 1;

    println!("Write a number: \n");

    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line!");

    println!("\nYou picked: {x}\n");

    let x: u32 = x.trim().parse().expect("Please write a number!");

    while counter < 21 {

        println!("{}", x * counter);

        counter += 1;
    }
}
