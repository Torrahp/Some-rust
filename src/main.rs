fn main(){
    greet();

    let c = 5;
    let b = 11;

    let sum = add(c, b);
    println!("The sum of {} and {} is: {}", c, b, sum);

    let side = 10;
    println!("The area of a rectangle with side {} is: {}",side, rectangle_area(side));

    let mut count = 0;

    loop {
        count +=1;
        println!("{} ", count);
        if count == 7{
            println!("Let break the loop at count = {}", count);
            break;
        }
    }

    for i in 1..=10{
        println!("Current number is: {}", i)
    }
}

fn greet(){
    println!("Hello, Rust!");
}

fn add(a:i32, b:i32)-> i32{
    a + b
}

fn rectangle_area(side:i32)->i32{
    side * side
}