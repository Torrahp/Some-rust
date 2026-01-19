use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Please enter your Target Number (1-100):");

    // --- ส่วนรับ Input ---
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    
    // เปลี่ยนชื่อตัวแปรให้ตรงความจริง: นี่คือ "เป้าหมาย"
    let target: i32 = input.trim().parse().expect("Number only!");

    // --- ส่วนเตรียมตัวแปร ---
    let mut low = 1;
    let mut high = 100;

    // --- ลูปการทาย ---
    loop {
        // 1. ย้ายการคำนวณมาไว้ตรงนี้ (เขียนครั้งเดียว ใช้ได้ตลอด)
        let computer_guess = (low + high) / 2;

        println!("Computer guesses: {}", computer_guess);

        // 2. เปรียบเทียบ เป้าหมาย vs สิ่งที่คอมทาย
        match target.cmp(&computer_guess) {
            Ordering::Less => {
                // เป้าหมาย < คอมทาย แปลว่า คอมทาย "มากไป"
                println!("-> Too High! (Reducing ceiling)"); 
                high = computer_guess - 1; 
            },
            Ordering::Greater => {
                // เป้าหมาย > คอมทาย แปลว่า คอมทาย "น้อยไป"
                println!("-> Too Low! (Raising floor)");
                low = computer_guess + 1;
            },
            Ordering::Equal => {
                println!("🎉 CORRECT! The number is {}", computer_guess);
                break; // จบเกม
            }
        }

        // 3. (แถม) ป้องกันกรณีหาไม่เจอ (เช่น ใส่เลข 200 หรือ -5)
        if low > high {
            println!("Wait... something is wrong. Number not found!");
            break;
        }
    }
}