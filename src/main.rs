use rand::Rng;
use std::io;

fn main() {
    println!("Provide the correct math result!");

    let num_1 = rand::thread_rng().gen_range(1, 101);
    let num_2 = rand::thread_rng().gen_range(1, 101);

    let result = num_1 + num_2;

    //println!("Num 1 is : {}", num_1);
    //println!("Num 2 is : {}", num_2);

    loop {
        let mut user_answer = String::new();

        println!("What is the result of : {} + {} ?", num_1, num_2);

        println!("Type here...");

        io::stdin()
            .read_line(&mut user_answer)
            .expect("Error reading your answer");

        let user_answer: u32 = match user_answer.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please provide a number!");
                continue;
            }
        };

        if user_answer == result {
            println!("You won!");
            break;
        } else {
            println!("Wrong!");
            continue;
        }
    }
}
