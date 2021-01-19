use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Let's begin");
    let number = rand::thread_rng().gen_range(0, 101);
    
    loop
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Wrong input");
        let input: u32 = match input.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Your answer is: {}", input);
        match input.cmp(&number)
        {
            Ordering::Less => println!("Too low"),
            Ordering::Equal => { println!("Right"); break; },
            Ordering::Greater => println!("Too much"),
        }
    }
}
