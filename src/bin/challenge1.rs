use std::io;

fn main() {
    println!("Enter the desired numerical value.");

    let mut number_input = String::new(); 

    io::stdin()
        .read_line(&mut number_input)
        .expect("Failed to read line");

    let number: i32 =number_input
        .trim()
        .parse()
        .expect("Please type a number!");

    for i in 1..=number{
        fizzbuzz(i);
    }
}

/*fn fizzbuzz(n: i32){
    if n % 3 == 0 && n % 5 == 0{
        println!("FizzBuzz");
    }else if n % 3 ==0 {
        println!("Fizz");
    }else if n % 5 ==0 {
        println!("Buzz");
    }else{
        println!("{}", n)
    }
}*/

fn fizzbuzz(n: i32){
    match (n % 3, n % 5){
        (0, 0) => println!("FizzBuzz"),
        (0, _) => println!("Fizz"),
        (_, 0) => println!("Buzz"),
        _ => println!("{}", n),
    }
}

