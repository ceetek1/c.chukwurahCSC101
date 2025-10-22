use std::io;

fn main() {
    println!("\nstudent Information management Stystem!");

    // input name
    println!("\n Please Enter your name.");
    let mut name = String::new();
        io::stdin()
        .read_line(&mut name).expect("Failed to read Input");
    println!("Your name is: {}", name);
    //input age 
    println!("\n Enter your age.");
    let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Failed to Read input");
    let age :i32 = age. trim().parse().expect("Input not an intiger ");
    println!("Your Age is : {}", age);    






}

