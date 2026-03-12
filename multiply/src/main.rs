use std::io;

fn main() {

    let mut x = String::new();
    let mut counter: u32 = 1;
    let placeholder: u32 = 0;

    println!("\nPlease type a number to multiply: \n");

    io::stdin()
        .read_line(&mut x)
        .expect("Error!");

    let x: u32 = x.trim().parse().expect("Please type a number!");

    println!("\nYou picked: {x}\n");
    println!("Here are the multiples of: {x}\n");

    while counter < 21 {

        if counter < 10 {
            println!("{placeholder}{counter})  {}", x * counter);
        }

        else {
            println!("{counter})  {}", x* counter);
        }

        counter += 1;
    }

}
